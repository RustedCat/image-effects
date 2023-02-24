use std::io::Cursor;

use base64::engine::{general_purpose as gpe, Engine as _};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"grayscale called".into());

    let base64_to_vector = gpe::STANDARD_NO_PAD.decode(encoded_file).unwrap();

    log(&"image decoded".into());

    let img = load_from_memory(&base64_to_vector).unwrap();
    log(&"image loaded".into());

    let img = img.grayscale();
    log(&"grayscale effect applied".into());

    let mut buffer = vec![];
    img.write_to(&mut Cursor::new(&mut buffer), Png).unwrap();
    log(&"new image created".into());

    let encoded_img = gpe::STANDARD_NO_PAD.encode(buffer);
    let data_url = format!("data:image/png;base64,{}", encoded_img);

    data_url
}
