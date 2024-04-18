File Archiver - A Versatile File Encoder/Decoder

Introduction

File Archiver is a Rust command-line tool designed to efficiently encode and decode files for video archiving. It offers flexibility in handling various input file formats and converting them into encoded video outputs or individual image frames, promoting long-term preservation.

Key Features

    Encoding and Decoding: Supports both encoding files into a video format (currently under development) and decoding encoded files back into their original format.
    Format Agnostic: Can encode various file types, providing a more generic solution for archiving different content.
    Frame Extraction: Enables creating individual image frames from the encoded file for enhanced visualization and analysis (requires encoding first).
    Resolution Control: Allows you to specify the desired output resolution (FHD, HD, SD, or custom) for both video and frames, ensuring compatibility with different display needs.

Installation

Prerequisites:

    Rust (version 1.56 or later recommended) installed on your system. You can find installation instructions at https://www.rust-lang.org/tools/install.

From Source:

    Clone the repository:
    Bash

    git clone https://github.com/pascall-de-creator/yt-data-drive.git

    Use code with caution.

Navigate to the project directory:
Bash

cd yt-data-drive

Use code with caution.

Build the project:
Bash

cargo build --release

Use code with caution.

This will create an executable named yt-data-drive in the target/release directory.

Usage

Basic Usage:
Bash

yt-data-drive file.txt --mode encode --output output.avi

Use code with caution.

This encodes the file.txt and saves the output video as output.avi (encoding is under development).

Available Options:
Option	Short Flag	Long Flag	Description	Required	Default Value
Mode	-m	--mode	Encode or decode the file (encode/decode)	Yes	-
Input File			The file to be encoded/decoded	Yes	-
Output File	-o	--output	The output filename (with extension for video)	No	output.png
Resolution	-r	--resolution	Output resolution (FHD, HD, SD, or custom)	No	HD
No Video		--no-video	Do not generate video output (only for encoding)	No	false
No Frames		--no-frames	Do not generate image frames (only for encoding)	No	true

Examples:

    Encode a file and save individual frames:
    Bash

    yt-data-drive image.jpg --mode encode --output encoded_image --no-video --no-frames=false

    Use code with caution.

Decode an encoded file (assuming decoding is implemented):
Bash

yt-data-drive encoded_file.avi --mode decode --output decoded_file.jpg

Use code with caution.

Disclaimer

    Video encoding functionality is currently under development and might not be fully functional yet.

Contributing

We welcome contributions to this project! If you have improvements, new features, or bug fixes, please consider creating a pull request.

Getting Help

    Feel free to open issues on the GitHub repository for any questions or problems.
