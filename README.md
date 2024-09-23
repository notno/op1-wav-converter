# op1-wav-converter
Converts Teenage Engineering OP-1 32bit PCM integer WAV files to 32bit float WAV files for e.g. Ableton Live

# Background
The Teenage Engineering OP-1 Field exports 32bit PCM integer WAV files. These files are not directly compatible with Ableton Live, which expects 32bit float WAV files. This is a small utility to convert all `.wav` files in a directory and subdirectories to 32bit float WAV files using FFMPEG.

This little program will recursively search for all files with the extension `.wav`, starting in the current directory, and use FFMPEG to convert them to 32bit float WAV files, (leaving the originals in place).

I have only tested it on Windows 11 Powershell. Let me know if you try it on Linux or MacOS!

This could have been just a shell script, but I wanted to learn Rust, so I made it in Rust.

# Usage
Conceivably you can build it yourself:
```
cargo build --release
```

Then you should add it to your PATH, or install it somewhere in your PATH.

Run it from the root directory of a file hierarchy containing `.wav` files you want to convert.

Copy the wav files from the OP-1 Field (e.g. the entire `tapes` directory) to your computer's hard drive, then open a terminal and navigate to that directory. Run the program from the terminal.

# Dependencies
- [FFMPEG](https://ffmpeg.org/download.html) must be installed and available in the PATH.
