use std::fs::File;
use std::io::{BufWriter, Write};

use rt::f32_buf_to_u8;

// Writes a u8 data buffer in RGBA format to a png file
pub fn write_png(file_name: &str, width: u32, height: u32, data: &[f32]) {
    let file = File::create(format!("{}.png", file_name)).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let u8_data = f32_buf_to_u8(data);
    writer.write_image_data(&u8_data).unwrap(); // Save
}

pub fn write_pfm(file_name: &str, width: u32, height: u32, data: &[f32]) {
    let file = File::create(format!("{}.pfm", file_name)).unwrap();
    let ref mut writer = BufWriter::new(file);

    writer
        .write(format!("PF\n{} {}\n-1.0\n", width, height).as_bytes())
        .expect("Failed to write pfm header");

    // data stores the image L->R T->B
    // pfm format is L->R B->T
    // So flip and then split into le bytes
    let le_bytes: Vec<u8> = data
        // Flip
        .chunks((width * 3) as usize)
        .rev()
        .flatten()
        // Convert to bytes
        .flat_map(|&v| v.to_le_bytes())
        .collect();

    writer
        .write_all(&le_bytes)
        .expect("Failed to write float data");
}
