
#[macro_use]
mod utils;
mod constants;
mod matrix;
mod types;

use utils::*;
use constants::*;
use matrix::*;
use types::*;

use chrono::{DateTime, Local, Duration};
use wasm_bindgen::prelude::*;
use web_sys::ImageData;
use image::{RgbaImage, Rgba};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;


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
    let mut matrix: MatrixType = [[0; CELL_FOR_SIDE_USIZE]; CELL_FOR_SIDE_USIZE];
    let mut matrix_cnt: MatrixType = [[0; CELL_FOR_SIDE_USIZE]; CELL_FOR_SIDE_USIZE];
    //let mut matrix: MatrixArcType = Arc::new(Mutex::new(matrix));

    console_log!("INITIAL SUM: {:?}", matrix_count_alive(&matrix));

    let epoch_text = get_paragraph("epoch");
    let epoch_total_text = get_paragraph("epoch-total");
    let time_text = get_paragraph("time");
    let cell_total_text = get_paragraph("cell-total");


    let canvas = get_canvas("canvas");
    let context = get_canvas_context(&canvas);

    canvas.set_width(SIZE);
    canvas.set_height(SIZE);

    let mut image: image::ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(SIZE, SIZE);
    let image_raw = image.as_raw();

    let image_array = image_raw.as_slice();
    let image_clamped_array = wasm_bindgen::Clamped(image_array);

    let imagedata = ImageData::new_with_u8_clamped_array(image_clamped_array, SIZE).unwrap();

    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    context.put_image_data(&imagedata, 0.0, 0.0).unwrap();

    reset_all(&mut image, &context, &imagedata);
    matrix_random_fill(&mut image, &mut matrix, START_CELL);

    let mut epoch: usize = 0;
    let mut temp: DateTime<Local>;
    let mut _delta: Duration;
    let mut now: DateTime<Local>;

    let start = chrono::offset::Local::now();
    let mut count = matrix_count(&matrix_cnt);
    let mut seconds_from_start: f64;
    

    while count <= (CELL_FOR_SIDE*CELL_FOR_SIDE) {
        let mut total_alive: u32 = 0;
        temp = chrono::offset::Local::now();

        cell_count_neighbors(&mut matrix, &mut matrix_cnt, &mut total_alive);
        draw_matrix(&mut image, &matrix, &mut total_alive);
        update_canvas(&context, &imagedata);
        js_sleep(CLOCK as i32).await.unwrap();
    
        epoch += 1;
        now = chrono::offset::Local::now();
        seconds_from_start = ((now - start).num_milliseconds()) as f64/1000.0 as f64;

        if epoch % (FPS as usize/16) == 0 && count < (CELL_FOR_SIDE*CELL_FOR_SIDE) - 3{
            epoch_text.set_text_content(Option::from(format!("EPOCH (AVG): {:.2}/s", epoch as f64/seconds_from_start).as_str()));
        }

        if epoch % (FPS as usize/8) == 0 {
            _delta = temp - now;
            if  count < (CELL_FOR_SIDE*CELL_FOR_SIDE) - 3{
                time_text.set_text_content(Option::from(format!("TIME: {} s", seconds_from_start).as_str()));
    
                epoch_total_text.set_text_content(Option::from(format!("EPOCH TOTAL: {}", epoch).as_str()));
            }else{
                count = CELL_FOR_SIDE*CELL_FOR_SIDE;
                time_text.set_class_name("colored");
            }
            cell_total_text.set_text_content(Option::from(format!("CELLS: {}/{}", count, CELL_FOR_SIDE*CELL_FOR_SIDE).as_str()));
        }

        count = matrix_count(&matrix_cnt);

        /* if count >= (CELL_FOR_SIDE*CELL_FOR_SIDE) - 3 {
            draw_matrix_black(&mut image);
        } */
    }
    time_text.set_class_name("colored");
}