use std::fs::File;
use std::io::Read;
use image::{ RgbImage };
use spinners::{Spinner, Spinners};

fn file_to_bits(filename: &str) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut file = File::open(filename).expect("Error opening file");

    file.read_to_end(&mut buffer).expect("Error reading file");
    buffer
}

fn add_header(bits: &Vec<u8>, filename: &str) -> Vec<u8> {
    let mut header_buffer: Vec<u8> = Vec::new();
    let filename_bytes: &[u8] = filename.as_bytes();
    let filename_length: [u8; 2] = (filename_bytes.len() as u16).to_ne_bytes();

    header_buffer.extend_from_slice(&[filename_bytes, &filename_length].concat());
    header_buffer.extend_from_slice(&bits);
    header_buffer
}

fn bits_to_pixels(bits: &Vec<u8>) -> Vec<[u8; 3]> {
    let mut pixels = Vec::new();
    let mut new_bits = Vec::<u8>::new();

    new_bits.extend_from_slice(&[[0].repeat(3 - (bits.len() % 3)), (&bits).to_vec()].concat());

    for i in 0..(new_bits.len() / 3) {
        pixels.push([new_bits[i * 3], new_bits[i * 3 + 1], new_bits[i * 3 + 2]]);
    }

    pixels
}

fn pixels_to_img(pixels: &Vec<[u8; 3]>, reslution: (u16, u16)) -> Result<RgbImage, String> {
    let mut image = RgbImage::new(reslution.0.into(), reslution.1.into());

    for (i, pixel) in pixels.iter().enumerate() {
        let pixel = image::Rgb(*pixel);
        let x = i % reslution.0 as usize;
        let y = i / reslution.0 as usize;

        image.put_pixel(x as u32, y as u32, pixel);
    }

    Ok(image)
}

// fn create_video() {
//
// }

pub fn run(file: &str, output: &str, resolution: (u16, u16), no_video: bool, no_frames: bool) {
    let mut sp = Spinner::new(Spinners::Dots9, "Encoding File...".into());

    let bits = add_header(&file_to_bits(file), file);
    let pixels = bits_to_pixels(&bits);
    let estimated_frames = bits.len() / (3 * resolution.0 as usize * resolution.1 as usize) + 1;

    if !no_frames {
        for frame_num in 0..estimated_frames {
            let frame_size: u128 = (resolution.0 * resolution.1) as u128;
            let start = frame_num as u128 * frame_size;
            let end = (start + frame_size).min(pixels.len() as u128);

            let frame_pixels: Vec<[u8; 3]> = (&pixels[start as usize..end as usize]).to_vec();

            let frame_output = format!("{}_{}.png", output, frame_num);
            let image = pixels_to_img(&frame_pixels, resolution) .expect("Failed to render frame");
            image.save(&frame_output).expect("Failed to save frame");
        }
    }

    // if !no_video { unimplemented!() }

    sp.stop_with_newline();
}
