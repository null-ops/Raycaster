mod vec2;

use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

// When the wasm module is loaded, this function will run
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Get window and document
    let window = window().unwrap();
    let document = window.document().unwrap();

    // Get canvas element by id "canvas"
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Let's draw a single red pixel at (10, 10)

    // Create a 1x1 pixel ImageData with red color (RGBA)
    let mut data = vec![]; // red pixel fully opaque
    for i in 0..1 {
        data.push(255);
        data.push(0);
        data.push(0);
        data.push(255);
    }
    let image_data = ImageData::new_with_u8_clamped_array_and_sh( 
        wasm_bindgen::Clamped(&data), 1, 1)?;

    // Put pixel at position (10, 10)
    context.put_image_data(&image_data, 10.0, 10.0)?;

    Ok(())
}