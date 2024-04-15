mod encoder;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about = "Rust image encoder with resolution and output options", long_about = None)]
struct Args {
    file: String,

    #[arg(short, long, help = "Output file (defaults to output.png)")]
    output: Option<String>,

    #[arg(short, long, help = "Output resolution (defaults to 1920x1080)")]
    resolution: Resolution,
}

#[derive(Clone, Debug, PartialEq, ValueEnum)]
enum Resolution {
    Fhd,
    Hd, // This is now the default
    Sd,
}

impl Resolution {
    fn to_dimensions(&self) -> (u16, u16) {
        match self {
            Resolution::Fhd => (3840, 2160),
            Resolution::Hd => (1920, 1080), // Default resolution
            Resolution::Sd => (1280, 720),
        }
    }
}

fn main() {
    let args = Args::parse();
    let output_file: String = args.output.unwrap_or_else(|| "output.png".to_string());
    let (width, height) = args.resolution.to_dimensions();

    encoder::run(&args.file, &output_file, (width, height))
}
