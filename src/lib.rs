use wasm_bindgen::prelude::*;
use image::ImageOutputFormat;
use std::io::Cursor;

#[wasm_bindgen]
pub fn process_image(data: &[u8]) -> *const u8 {
    let img = image::load_from_memory(data).expect("Failed to load image");

    let mut compressed_data = Cursor::new(Vec::new());
    img.write_to(&mut compressed_data, ImageOutputFormat::Jpeg(75)).expect("Failed to write image");

    let vec = compressed_data.into_inner();
    let ptr = vec.as_ptr(); 
    std::mem::forget(vec);

    ptr 
}

#[wasm_bindgen]
pub fn get_compressed_size(data: &[u8]) -> usize {
    let img = image::load_from_memory(data).expect("Failed to load image");

    let mut compressed_data = Cursor::new(Vec::new());
    img.write_to(&mut compressed_data, ImageOutputFormat::Jpeg(75)).expect("Failed to write image");

    let vec = compressed_data.into_inner();
    vec.len() 
}
