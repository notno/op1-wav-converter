# op1-wav-converter
Converts Teenage Engineering OP-1 32bit PCM integer WAV files to 32bit float WAV files for e.g. Ableton Live. Now includes the ability to trim silence from WAV files.

# Features
- Converts 32bit PCM integer WAV files to 32bit float WAV files.
- Optionally trims silence from the end of WAV files using the `--trim` flag.
- Works on all files in this directory and all its children, or optionally specify a single file to convert using the `-f` or `--file` flag.

# Background
The Teenage Engineering OP-1 Field exports 32bit PCM integer WAV files. These files are not directly compatible with Ableton Live, which expects 32bit float WAV files. This is a small utility to convert all `.wav` files in a directory and subdirectories to 32bit float WAV files.

This little program will recursively search for all files with the extension `.wav`, starting in the current directory, and convert them to 32bit float WAV files, (leaving the originals in place).

**I have only tested it on Windows 11 Powershell**. Let me know if you try it on Linux or MacOS!

# Usage
Always have backups of your OP-1 wav files before using this. If this half-baked little program mangles your work, I'm sorry. I'm not responsible for any lost data.

That said, you can download the latest Windows release from the [releases page](https://github.com/notno/op1-wav-converter/releases).

Or conceivably you can build it yourself if you have Rust installed. Clone the repository, navigate to the root directory of the repository, and run:
```
cargo build --release
```

Then you should add it to your PATH, or install it somewhere in your PATH.

**Run it from the root directory of a file hierarchy containing `.wav` files you want to convert.**

What I do is copy the wav files from the OP-1 Field (e.g. the entire `tapes` directory) to my computer's hard drive, then open a command prompt and navigate to that directory (e.g. `cd C:\Users\Dude\MyJams`). Then run the program from the terminal: `op1-wav-converter`.

To convert and trim silence from all `.wav` files in the current directory and subdirectories:
```
op1-wav-converter --trim
```

To specify a single WAV file for processing, use the `-f` or `--file` flag:
```
op1-wav-converter -f path/to/your/file.wav
```

# Command Line Options
- `-f`, `--file`: Specify a single file to convert.
- `--trim`: Trim silence from the end of the WAV files.
