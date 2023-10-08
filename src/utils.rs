use crate::constants::*;

use wasm_bindgen::prelude::*;
use web_sys::HtmlButtonElement;
use web_sys::HtmlCanvasElement;
use web_sys::HtmlParagraphElement;
use web_sys::{ImageData, CanvasRenderingContext2d};
use image::{RgbaImage, Rgba};

use js_sys::{Promise, Math::random};
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

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

pub fn get_canvas_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    return ctx;
}

pub fn get_canvas(id: &str) -> HtmlCanvasElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(id).unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    return canvas;
}

pub fn get_paragraph(id: &str) -> HtmlParagraphElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let paragraph = document.get_element_by_id(id).unwrap();
    let paragraph: web_sys::HtmlParagraphElement = paragraph
    .dyn_into::<web_sys::HtmlParagraphElement>()
    .map_err(|_| ())
    .unwrap();

    return paragraph;
}

pub fn get_button(id: &str) -> HtmlButtonElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let button = document.get_element_by_id(id).unwrap();
    let button: web_sys::HtmlButtonElement = button
    .dyn_into::<web_sys::HtmlButtonElement>()
    .map_err(|_| ())
    .unwrap();

    return button;
}