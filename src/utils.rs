use crate::constants::*;

use wasm_bindgen::prelude::*;
use web_sys::{ImageData, CanvasRenderingContext2d};
use image::{RgbaImage, Rgba};

use js_sys::{Promise, Math::random};
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use std::sync::{Arc, Mutex, MutexGuard};

pub type MatrixType = [[u8; CELL_FOR_SIDE_USIZE]; CELL_FOR_SIDE_USIZE];
pub type MatrixMutexType = Arc<Mutex<MatrixType>>;
pub type MatrixArcType = Arc<Mutex<MatrixType>>;

#[wasm_bindgen]
pub async fn js_sleep(time_ms: i32) -> Result<(), JsValue> {
    // Create a JavaScript Promise that resolves after 2 seconds
    let promise = Promise::new(&mut |resolve, _| {
        let window = window().unwrap();
        let closure = Closure::once_into_js(move || {
            resolve.call0(&JsValue::null()).unwrap();
        });
        window.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            time_ms, // 2000 milliseconds = 2 seconds
        ).unwrap();
    });

    // Convert the Promise to a Future and await it
    JsFuture::from(promise)
        .await
        .map(|_| ())
}

pub fn draw_background(img: &mut RgbaImage, r: u8, g: u8, b: u8, a: u8){
    for x in 0..SIZE {
        for y in 0..SIZE {
            img.put_pixel(x, y, Rgba([r, g, b, a]));
            img.put_pixel(y, x, Rgba([r, g, b, a]));
        }
    }
}


pub fn draw_grid(img: &mut RgbaImage, space: u32){
    let mut x = 0;

    while x < SIZE {
        for y in 0..SIZE{
            img.put_pixel(x, y, COLOR_GRID);
            img.put_pixel(y, x, COLOR_GRID);

        }
        x += space;
    }
}

pub fn fill_cell(image: &mut RgbaImage, i: u32, j: u32){
    for x in (i*CELL_PIXEL_SIZE)..((i + 1)*(CELL_PIXEL_SIZE)){
        for y in (j*CELL_PIXEL_SIZE)..((j + 1)*(CELL_PIXEL_SIZE)){
            image.put_pixel(x, y, COLOR_CELL);
        }
    }
}

pub fn update_canvas(context: &CanvasRenderingContext2d, imagedata: &ImageData){
    context.put_image_data(imagedata, 0.0, 0.0).unwrap();
}

pub fn reset_all(image: &mut RgbaImage, context: &CanvasRenderingContext2d, imagedata: &ImageData){
    let r_random = 255 - (random() * 245.0) as u8;
    let g_random = 255 - (random() * 10.0) as u8;
    let b_random = 255 - (random() * 245.0) as u8;

    draw_background(image, r_random, g_random, b_random, 255);
    draw_grid(image, SIZE/CELL_FOR_SIDE);
    update_canvas(&context, &imagedata);
}

pub fn draw_matrix(image: &mut RgbaImage, mutex: &mut MutexGuard<'_, [[u8; CELL_FOR_SIDE_USIZE]; CELL_FOR_SIDE_USIZE]>, total_alive: &mut u32){
    for (i, row) in mutex.iter().enumerate(){
        for (j, cell) in row.iter().enumerate() {
            if *cell == 1 {
                *total_alive += 1;
                fill_cell(image, i as u32, j as u32);
            }
        }
    }
}

pub fn draw_matrix2(image: &mut RgbaImage,  matrix: &MatrixArcType){
    let mutex = matrix.lock().unwrap();

    for (i, row) in mutex.iter().enumerate(){
        for (j, cell) in row.iter().enumerate() {
            if *cell == 1 {
                fill_cell(image, i as u32, j as u32);
            }
        }
    }
}

pub fn matrix_random_fill(image: &mut RgbaImage, matrix: &MatrixArcType, max: usize){
    let mutex = &mut matrix.lock().unwrap();

    let mut randx = (random()*CELL_FOR_SIDE as f64) as usize;
    let mut randy = (random()*CELL_FOR_SIDE as f64) as usize;

    for _ in 0..max {
        randx = (random()*CELL_FOR_SIDE as f64) as usize;
        randy = (random()*CELL_FOR_SIDE as f64) as usize;

        mutex[randx][randy] = 1;
    }

    let mut t: u32 = 0;
    draw_matrix(image, mutex, &mut t);
}

pub fn matrix_count_alive(matrix: &MatrixArcType) -> u32 {
    let mutex = &mut matrix.lock().unwrap();

    return mutex.iter().flatten().sum::<u8>() as u32;
}



pub fn cell_count_neighbors(image: &mut RgbaImage, matrix: &MatrixArcType, total_alive: &mut u32) {
    let mutex = &mut matrix.lock().unwrap();
    let mut count;
    let mut is_alive;
    let mut x: i32;
    let mut y: i32;

    for (i, row) in mutex.clone().iter().enumerate(){
        for (j, cell) in row.iter().enumerate() {
            count = 0;
            is_alive = *cell == 1;

            let mut k: usize = 0;
            while k < 8 && count < 4 {
                x = (i as i32 + MATRIX_INDEX_CHECKS[k][0] as i32) as i32;
                y = (j as i32 + MATRIX_INDEX_CHECKS[k][1] as i32) as i32;

                if (x >= 0 && x < CELL_FOR_SIDE_USIZE as i32)  
                    && (y >= 0 && y < CELL_FOR_SIDE_USIZE as i32) {

                   if mutex[x as usize][y as usize] == 1 {
                        count += 1;

                    }
                }

                k += 1;    
            }

            if is_alive && count < 2 {
                mutex[i][j] = 0;
            }else if is_alive && count > 3 {
                mutex[i][j] = 0;
            }else if !is_alive && count == 3 {
                mutex[i][j] = 1;
            }
        }
    }

    draw_matrix(image, mutex, total_alive);
}


pub fn matrix_count(matrix: &MatrixArcType) -> u32{
    let mutex = matrix.lock().unwrap();
    let mut s: u32 = 0;
    
    for row in mutex.iter(){
        for cell in row.iter(){
            if *cell == 1{
                s += 1;
            }
        }
    }

    return s;
}