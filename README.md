# YT Data Drive - A Versatile File Encoder/Decoder

## Introduction

YT data drive is a Rust command-line tool designed to efficiently encode and decode files for video archiving. It offers flexibility in handling various input file formats and converting them into encoded video outputs or individual image frames, promoting long-term preservation.

## Key Features

- **Encoding and Decoding** : Supports both encoding files into a video format and decoding encoded files back into their original format (Still in development).
- **Format Agnostic** : Can encode various file types, providing a more generic solution for archiving different content.
- **Frame Extraction** : Enables creating individual image frames from the encoded file for enhanced visualization and analysis (requires encoding first).
- **Resolution Control**: Allows you to specify the desired output resolution (FHD, HD, SD, or custom) for both video and frames, ensuring compatibility with different display needs.

## Installation

### Prerequisites:

`Rust (version 1.56 or later recommended).`

### From Source:

Clone the repository:

```bash
git clone https://github.com/pascall-de-creator/yt-data-drive.git
cd yt-data-drive
cargo build --release
yt-data-drive file.txt --mode encode --output output
```

This encodes the file.txt and saves the output video as output.mp4.

#### Available Options:

|Option        |Short Flag	|Long Flag	   |Description	                                      |Required  |Default Value
|-|-|-|-|-|-|
|Input File	   |	        |              |The file to be encoded/decoded	                  |Yes	     | -
|Mode	       |-m	        |--mode	       |Encode or decode the file (encode/decode)	      |Yes	     | -
|Output File   |-o	        |--output	   |The output filename (with extension for video)	  |No        | output.png
|Resolution	   |-r	        |--resolution  |Output resolution (FHD, HD, SD, or custom)	      |No	     | HD
|No Video      |            |--no-video	   |Do not generate video output (only for encoding)  |No	     | false
|No Frames	   |            |--no-frames   |Do not generate image frames (only for encoding)  |No  	     | true

## Disclaimer

Video decoding functionality is currently under development and might not be fully functional yet.

## Contributing

We welcome contributions to this project! If you have improvements, new features, or bug fixes, please consider creating a pull request.

## Getting Help

Feel free to open issues on the GitHub repository for any questions or problems.
