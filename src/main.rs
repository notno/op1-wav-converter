use hound::{WavReader, WavWriter, WavSpec, SampleFormat};
use std::path::Path;
use std::ffi::OsStr;
use walkdir::WalkDir;

fn main() {
    // Get the current directory
    let directory = std::env::current_dir().unwrap();

    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        if is_wav_file(&path){
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

