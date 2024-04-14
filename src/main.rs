mod encoder;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();

    const RESOLUTIONS: ((u16, u16), (u16, u16), (u16, u16)) = (
        (3840, 2160),
        (1920, 1080),
        (1280, 720), 
    );

    encoder::run(&args.file, &args.output, RESOLUTIONS.1)
}
