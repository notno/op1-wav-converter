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
        .get_matches();

    let trim = matches.contains_id("trim");
    let directory = std::env::current_dir().unwrap();

    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        if is_wav_file(&path) {
            if trim {
                println!("Trimming silence from {}...", path.display());
                if let Err(e) = trim_silence(&path, &path.with_extension("TRIM.wav")) {
                    eprintln!("Error trimming {}: {}", path.display(), e);
                }
            } else 
            if let Ok(is_32bit_pcm) = is_32bit_pcm_wav(&path) {
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

    // Create the WAV writer
    let mut writer = WavWriter::create(output_path, spec)?;

    // Read samples and trim silence
    let samples: Vec<i32> = reader.samples::<i32>().filter_map(Result::ok).collect();
    let trimmed_samples = trim_silence_from_samples(&samples);

    // Write trimmed samples
    for sample in trimmed_samples {
        writer.write_sample(*sample)?;
    }

    writer.finalize()?;
    Ok(())
}

fn trim_silence_from_samples(samples: &[i32]) -> &[i32] {
    let threshold = 1000; // Define a threshold for silence
    let mut end = samples.len();

    for (i, &sample) in samples.iter().enumerate().rev() {
        if sample.abs() > threshold {
            end = i + 1;
            break;
        }
    }

    &samples[..end]
}
