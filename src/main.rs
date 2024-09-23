use hound::WavReader;
use std::ffi::OsStr;
use std::path::{self, Path};
use std::process::Command;
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
                    if let Err(e) = convert_to_32bit_float(&path) {
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

fn convert_to_32bit_float(path: &Path) -> Result<(), std::io::Error> {
    let output_path = path.with_extension("float32.wav");
    let output_path_str = output_path.to_str().unwrap();

    // using ffmpeg
    
    let output = Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            path.to_str().unwrap(),
            "-c:a",
            "pcm_f32le",
            output_path_str,
        ])
        .output()?;
    
    if !output.status.success() {
        eprintln!("ffmpeg failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

