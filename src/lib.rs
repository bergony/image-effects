use std::fmt::format;

use base64::{decode, encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Grayscale Called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decode".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"image load".into());

    img = img.grayscale();

    log(&"Grayscale effect applie".into());

    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    log(&"new image written".into());

    let encode_img = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encode_img);
    data_url
}
