mod encoder;
mod decoder;

use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, PartialEq, ValueEnum)]
enum Mode { Encode, Decode }

#[derive(Clone, Debug, PartialEq, ValueEnum)]
enum Resolution { FHD, HD, SD, DD }

#[derive(Debug, Parser)]
#[command(version, about="A file - video encoder/decoder for video archiving", long_about=None)]
struct Args {
    file: String,

    #[arg(short, long, help="Choose mode")]
    mode: Option<Mode>,

    #[arg(short, long, help="Output file (defaults to 'output')")]
    output: Option<String>,

    #[arg(short, long, help="Output resolution (defaults to 1920x1080)")]
    resolution: Option<Resolution>,

    #[arg(long, help="Output video (defaults to false)")]
    no_video: bool,

    #[arg(long, help="Output frames (defaults to true)")]
    no_frames: bool,
}

fn main() {
    let args = Args::parse();
    let output_file: String = args.output.unwrap_or_else(|| "output.png".to_string());
    let (width, height) = match args.resolution {
        Some(resolution) => resolution.to_dimensions(),
        None => Resolution::HD.to_dimensions(),
    };

    if args.mode == Some(Mode::Decode) {
        decoder::run(&args.file, &output_file, (width, height), args.no_frames)
    } else {
        encoder::run(&args.file, &output_file, (width, height), args.no_video, args.no_frames)
    }
}

impl Resolution {
    fn to_dimensions(&self) -> (u16, u16) {
        match self {
            Resolution::FHD => (3840, 2160),
            Resolution::HD => (1920, 1080),
            Resolution::SD => (1280, 720),
            Resolution::DD => (20, 20),
        }
    }
}
