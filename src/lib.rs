use std::{f64, cell};
use wasm_bindgen::prelude::*;
use web_sys::{ImageData, CanvasRenderingContext2d};
use image::{RgbImage, Rgb, ImageFormat, RgbaImage, Rgba};

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

use js_sys::{Promise, Math::{random, self}, Date};
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
    
const SIZE: u32 = 256;
const USIZE: usize = SIZE as usize;
const FPS: u32 = 60;
const FPR_REFRESH_MS: f64 = 300.0;
const CELL_FOR_SIDE: u32 = 16;
const CELL_PIXEL_SIZE: u32 = SIZE/CELL_FOR_SIDE;

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

fn draw_background(img: &mut RgbaImage, r: u8, g: u8, b: u8, a: u8){
    for x in 0..SIZE {
        for y in 0..SIZE {
            img.put_pixel(x, y, Rgba([r, g, b, a]));
            img.put_pixel(y, x, Rgba([r, g, b, a]));
        }
    }
}

fn draw_grid(img: &mut RgbaImage, space: u32){
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    
    while x < SIZE {
        y = 0;
        while  y < SIZE {
            img.put_pixel(x, y, Rgba([0, 0, 0, 255]));
            img.put_pixel(y, x, Rgba([0, 0, 0, 255]));


            y += 1;
        }

        x += space;
    }
}

fn fill_cell(img: &mut RgbaImage, i: u32, j: u32){
    console_log!("CELL LEN: {}", CELL_PIXEL_SIZE);

    for x in (i*CELL_PIXEL_SIZE)..((i + 1)*(CELL_PIXEL_SIZE)){
        for y in (j*CELL_PIXEL_SIZE)..((j + 1)*(CELL_PIXEL_SIZE)){
            img.put_pixel(x, y, Rgba([0, 0, 0, 255]));
        }
    }
}

fn update_canvas(context: &CanvasRenderingContext2d, imagedata: &ImageData){
    context.put_image_data(imagedata, 0.0, 0.0).unwrap();
}



#[wasm_bindgen(start)]
async fn start() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let fps_text = document.get_element_by_id("fps").unwrap();
    let fps_text: web_sys::HtmlParagraphElement = fps_text
    .dyn_into::<web_sys::HtmlParagraphElement>()
    .map_err(|_| ())
    .unwrap();

    fps_text.set_text_content(Option::from("0.0"));
    
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(SIZE);
    canvas.set_height(SIZE);

    let mut image: image::ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(SIZE, SIZE);
    let image_raw = image.as_raw();
    console_log!("{:?}", image_raw);
    
    let image_array = image_raw.as_slice();
    let image_clamped_array = wasm_bindgen::Clamped(image_array);

    let imagedata = ImageData::new_with_u8_clamped_array(image_clamped_array, SIZE).unwrap();
    console_log!("{:?}", image_clamped_array);
    
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    context.put_image_data(&imagedata, 0.0, 0.0).unwrap();

    let r_random = (random() * 255.0) as u8;
    let g_random = (random() * 255.0) as u8;
    let b_random = (random() * 255.0) as u8;

    draw_background(&mut image, r_random, g_random, b_random, 255);

    let mut i = 0;
    let clock = 1000/FPS;


    let test = js_sys::Function::new_no_args("");
    canvas.set_onclick(Option::from(&test));

    draw_grid(&mut image, SIZE/CELL_FOR_SIDE);
    update_canvas(&context, &imagedata);

    let mut t1 = Date::now();
    let mut temp = Date::now();
    let mut delta = Math::abs(temp - Date::now());
    let mut fps_calc = FPS as f64 + ((1000.0 - (delta*FPS as f64))*FPS as f64)/1000.0;
    let mut fps_string = format!("FPS: {}", fps_calc);
    let mut now = Date::now();

    fill_cell(&mut image, 5, 3);

    loop {
        temp = Date::now();

        js_sleep(clock as i32).await.unwrap();
        update_canvas(&context, &imagedata);

        delta = Math::abs(temp - Date::now());
        fps_calc = FPS as f64 + ((1000.0 - (delta*FPS as f64))*FPS as f64)/1000.0;
        fps_string = format!("FPS: {}", fps_calc);

        now = Date::now();

        if now - t1 >= FPR_REFRESH_MS {
            t1 = now;
            fps_text.set_text_content(Option::from(fps_string.as_str()));
        }    
      
        i += 1;
    }
}