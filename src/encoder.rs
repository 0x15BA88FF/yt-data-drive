use std::fs::File;
use std::io::Read;
use image::{ RgbImage };

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
        let y = i / reslution.1 as usize;
        println!("{x}, {y}");
        
        image.put_pixel(x as u32, y as u32, image::Rgb([255, 255, 255]));//pixel);
    }

    Ok(image)
}

// fn create_video() {
//
// }

pub fn run(file: &str, resolution: (u16, u16)) {
    let bits = file_to_bits(file);
    let bits = add_header(&bits, file);
    let pixels = bits_to_pixels(&bits);
    pixels_to_img(&pixels, resolution).expect("Failed to render frame").save("base.png");
}
