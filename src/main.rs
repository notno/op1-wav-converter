use clap::{Arg, Command};
use hound::{WavReader, WavWriter, WavSpec, SampleFormat};
use std::path::Path;
use std::ffi::OsStr;
use walkdir::WalkDir;

fn main() {
    let matches = Command::new("op1_wav_converter")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Converts and trims WAV files")
        .arg(
            Arg::new("trim")
                .short('t')
                .long("trim")
                .help("Trim silence from the end of WAV files")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_parser(clap::value_parser!(String))
                .value_name("FILE")
                .help("Specify a single WAV file to process"),
        )
        .get_matches();

    let trim = matches.contains_id("trim");
    if let Some(file_path) = matches.get_one::<String>("file") {
        let path = Path::new(file_path);
        process_file(path, trim);
    } else {
        let directory = std::env::current_dir().unwrap();
        for entry in WalkDir::new(directory)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            process_file(path, trim);
        }
    }
}

fn process_file(path: &Path, trim: bool) {
    if is_wav_file(&path) {
        if trim {
            println!("Trimming silence from {}...", path.display());
            if let Err(e) = trim_silence(&path, &path.with_extension("TRIM.wav")) {
                eprintln!("Error trimming {}: {}", path.display(), e);
            }
        } else if let Ok(is_32bit_pcm) = is_32bit_pcm_wav(&path) {
            if is_32bit_pcm {
                println!("{} is a 32-bit PCM WAV file. Converting...", path.display());
                if let Err(e) = convert_pcm_to_float(&path, &path.with_extension("float32.wav")) {
                    eprintln!("Error converting {} to 32-bit float: {}", path.display(), e);
                }
            } else {
                println!("{} is not a 32-bit PCM WAV file", path.display());
            }
        }
    }
}

fn is_wav_file(path: &Path) -> bool {
    path.extension() == Some(OsStr::new("wav"))
}

fn is_32bit_pcm_wav(path: &Path) -> Result<bool, hound::Error> {
    let reader = WavReader::open(path)?;
    Ok(reader.spec().bits_per_sample == 32 && reader.spec().sample_format == hound::SampleFormat::Int)
}


fn convert_pcm_to_float(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Open the input WAV file
    let mut reader = WavReader::open(input_path)?;
    let spec = reader.spec();

    // Check if the input is 32-bit integer PCM
    if spec.bits_per_sample != 32 || spec.sample_format != SampleFormat::Int {
        return Err("Input file is not 32-bit integer PCM".into());
    }

    // Prepare the output WAV spec
    let output_spec = WavSpec {
        channels: spec.channels,
        sample_rate: spec.sample_rate,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };

    // Create the WAV writer
    let mut writer = WavWriter::create(output_path, output_spec)?;

    // Convert and write samples
    for sample in reader.samples::<i32>() {
        let int_sample = sample?;
        let float_sample = int_sample as f32 / std::i32::MAX as f32;
        writer.write_sample(float_sample)?;
    }

    writer.finalize()?;
    Ok(())
}

fn trim_silence(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Open the input WAV file
    let mut reader = WavReader::open(input_path)?;
    let spec = reader.spec();

    // Create the WAV writer with 32-bit float specification
    let output_spec = WavSpec {
        channels: spec.channels,
        sample_rate: spec.sample_rate,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };
    let mut writer = WavWriter::create(output_path, output_spec)?;

    // Read samples and trim silence
    let samples: Vec<i32> = reader.samples::<i32>().filter_map(Result::ok).collect();
    let trimmed_samples = trim_silence_from_samples(&samples, spec.channels);

    // Convert trimmed samples to 32-bit float and write them
    for sample in trimmed_samples {
        let float_sample = sample as f32 / std::i32::MAX as f32;
        writer.write_sample(float_sample)?;
    }

    writer.finalize()?;
    Ok(())
}

fn trim_silence_from_samples(samples: &[i32], channels: u16) -> Vec<i32> {
    let threshold = 1000; // Define a threshold for silence
    let mut trimmed_samples = vec![Vec::new(); channels as usize];

    // Separate samples into channels
    for (i, &sample) in samples.iter().enumerate() {
        trimmed_samples[i % channels as usize].push(sample);
    }

    // Trim silence from each channel
    for ch_samples in &mut trimmed_samples {
        let mut end = ch_samples.len();
        for (i, &sample) in ch_samples.iter().enumerate().rev() {
            if sample.abs() > threshold {
                end = i + 1;
                break;
            }
        }
        ch_samples.truncate(end);
    }

    // Recombine channels into interleaved format
    let mut interleaved_samples = Vec::new();
    let min_length = trimmed_samples.iter().map(|ch| ch.len()).min().unwrap_or(0);
    for i in 0..min_length {
        for ch in 0..channels as usize {
            interleaved_samples.push(trimmed_samples[ch][i]);
        }
    }

    interleaved_samples
}
