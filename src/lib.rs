
#[macro_use]
mod utils;
mod constants;

use utils::*;
use constants::*;

use chrono::{DateTime, Local, Duration};
use wasm_bindgen::prelude::*;
use web_sys::{ImageData, CanvasRenderingContext2d};
use image::{RgbaImage, Rgba};
use std::sync::{Arc, Mutex};


#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen(start)]
async fn start() {
    let matrix: MatrixType = [[0; CELL_FOR_SIDE_USIZE]; CELL_FOR_SIDE_USIZE];
    let mut matrix: Arc<Mutex<MatrixType>> = Arc::new(Mutex::new(matrix));

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let epoch_text = document.get_element_by_id("epoch").unwrap();
    let epoch_text: web_sys::HtmlParagraphElement = epoch_text
    .dyn_into::<web_sys::HtmlParagraphElement>()
    .map_err(|_| ())
    .unwrap();


    let epoch_total_text = document.get_element_by_id("epoch-total").unwrap();
    let epoch_total_text: web_sys::HtmlParagraphElement = epoch_total_text
    .dyn_into::<web_sys::HtmlParagraphElement>()
    .map_err(|_| ())
    .unwrap();


    let time_text = document.get_element_by_id("time").unwrap();
    let time_text: web_sys::HtmlParagraphElement = time_text
    .dyn_into::<web_sys::HtmlParagraphElement>()
    .map_err(|_| ())
    .unwrap();

    let cell_total_text = document.get_element_by_id("cell-total").unwrap();
    let cell_total_text: web_sys::HtmlParagraphElement = cell_total_text
    .dyn_into::<web_sys::HtmlParagraphElement>()
    .map_err(|_| ())
    .unwrap();


    let cell_alive_text = document.get_element_by_id("cell-alive").unwrap();
    let cell_alive_text: web_sys::HtmlParagraphElement = cell_alive_text
    .dyn_into::<web_sys::HtmlParagraphElement>()
    .map_err(|_| ())
    .unwrap();


    cell_total_text.set_text_content(Option::from(format!("TOTAL CELLS: {}", CELL_FOR_SIDE*CELL_FOR_SIDE).as_str()));

    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(SIZE);
    canvas.set_height(SIZE);

    let mut image: image::ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(SIZE, SIZE);
    let image_raw = image.as_raw();

    let image_array = image_raw.as_slice();
    let image_clamped_array = wasm_bindgen::Clamped(image_array);

    let imagedata = ImageData::new_with_u8_clamped_array(image_clamped_array, SIZE).unwrap();
    //console_log!("{:?}", image_clamped_array);
    
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    context.put_image_data(&imagedata, 0.0, 0.0).unwrap();

    reset_all(&mut image, &context, &imagedata);
    matrix_random_fill(&mut image, &matrix, START_CELL);

    let mut epoch: usize = 0;
    let mut temp: DateTime<Local>;
    let mut _delta: Duration;
    let mut now: DateTime<Local>;

    let start = chrono::offset::Local::now();
    let mut total_alive: u32 = 0;
    
    loop {
        temp = chrono::offset::Local::now();

        cell_count_neighbors(&mut image, &mut matrix, &mut total_alive);
        
        update_canvas(&context, &imagedata);
        js_sleep(CLOCK as i32).await.unwrap();
    
        epoch += 1;

        if epoch % (FPS as usize/8) == 0 {
            now = chrono::offset::Local::now();
            _delta = temp - now;
            let seconds_from_start = ((now - start).num_milliseconds()) as f64/1000.0 as f64;

            epoch_text.set_text_content(Option::from(format!("EPOCH: {:.2}/s", epoch as f64/seconds_from_start).as_str()));
            time_text.set_text_content(Option::from(format!("TIME: {} s", seconds_from_start).as_str()));
        }

        if epoch % (FPS as usize) == 0{

            epoch_total_text.set_text_content(Option::from(format!("EPOCH TOTAL: {}", epoch).as_str()));
            
        }


        if epoch % (FPS as usize*2) == 0 {
            let cnt = matrix_count(&matrix.clone());
            cell_alive_text.set_text_content(Option::from(format!("ALIVE CELLS: {}", cnt).as_str()));
            console_log!("{:?}", matrix.lock().unwrap());
        }
    }
}