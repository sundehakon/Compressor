use wasm_bindgen::prelude::*;
use image::ImageOutputFormat;
use std::io::Cursor;

#[wasm_bindgen]
pub fn process_image(data: &[u8]) -> Vec<u8> {
    let img = image::load_from_memory(&data).expect("Failed to load image");

    let mut compressed_data = Cursor::new(Vec::new());
    img.write_to(&mut compressed_data, ImageOutputFormat::Jpeg(75)).expect("Failed to write image");

    compressed_data.into_inner()
}