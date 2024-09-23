# op1-wav-converter
Converts Teenage Engineering OP-1 32bit PCM integer WAV files to 32bit float WAV files for e.g. Ableton Live

# Background
The Teenage Engineering OP-1 Field exports 32bit PCM integer WAV files. These files are not directly compatible with Ableton Live, which expects 32bit float WAV files. This is a small utility to convert all `.wav` files in a directory and subdirectories to 32bit float WAV files.

This little program will recursively search for all files with the extension `.wav`, starting in the current directory, and convert them to 32bit float WAV files, (leaving the originals in place).

**I have only tested it on Windows 11**. Let me know if you try it on Linux or MacOS!

# Usage
You can download the latest Windows release from the [releases page](https://github.com/notno/op1-wav-converter/releases).

Or conceivably you can build it yourself if you have Rust installed. Clone the repository, navigate to the root directory of the repository, and run:
```
cargo build --release
```

Then you should add it to your PATH, or install it somewhere in your PATH.

**Run it from the root directory of a file hierarchy containing `.wav` files you want to convert.**

What I do is copy the wav files from the OP-1 Field (e.g. the entire `tapes` directory) to my computer's hard drive, then open a command prompt and navigate to that directory (e.g. `cd C:\Users\Dude\MyJams`). Then run the program from the terminal: `op1-wav-converter`.
