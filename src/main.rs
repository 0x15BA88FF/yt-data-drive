mod encoder;

use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Rust image encoder with resolution and output options", long_about = None)]
struct Args {
    file: String,

    #[arg(short, long, help = "Output file (defaults to output.png)")]
    output: Option<String>,

}

fn main() {
    let args = Args::parse();
    let resolution: ((u16, u16), (u16, u16), (u16, u16)) = ( (3840, 2160), (1920, 1080), (1280, 720), );
    let output_file: String = args.output.unwrap_or_else(|| "output.png".to_string());

    encoder::run(&args.file, &output_file, resolution.1)
}
