use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::ImageData;
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

#[wasm_bindgen]
pub fn sleep(ms: i32) -> js_sys::Promise {
    js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    })
}

#[wasm_bindgen]
pub fn process_raw_image(raw_data: Vec<u8>, width: u32, height: u32, gamma: f32) -> Vec<u8> {
    let mut result = Vec::with_capacity((width * height) as usize);

    for pixel in raw_data {
        let normalized_pixel = pixel as f32 / 255.0;
        let linear_pixel = normalized_pixel.powf(gamma);
        let corrected_pixel = (linear_pixel * 255.0) as u8;
        result.push(corrected_pixel);
    }

    result
}

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


#[wasm_bindgen(start)]
async fn start() {
    
    const WIDTH: u32 = 512;
    const HEIGHT: u32 = 512;

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(WIDTH + 100);
    canvas.set_height(HEIGHT + 100);

    let mut img = RgbaImage::new(WIDTH, HEIGHT);

    for x in 100..300 {
        img.put_pixel(x, x, Rgba([255, 120, 0, 255]));
    }

    let export_vet_image = img.as_raw();
    console_log!("{:?}", export_vet_image);
    let data = export_vet_image.as_slice();
    
    let clamped_image = wasm_bindgen::Clamped(data);

    let imagedata = ImageData::new_with_u8_clamped_array(clamped_image, WIDTH).unwrap();
    console_log!("{:?}", clamped_image);
    
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    context.put_image_data(&imagedata, 0.0, 0.0).unwrap();

    let mut i = 0;
    let clock = 1000/2;
    loop {
        let r_random = (random() * 255.0) as u8;
        let g_random = (random() * 255.0) as u8;
        let b_random = (random() * 255.0) as u8;

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                img.put_pixel(x, y, Rgba([r_random, g_random, b_random, 255]));
                img.put_pixel(y, x, Rgba([r_random, g_random, b_random, 255]));
            }
        }

        //context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        context.put_image_data(&imagedata, 0.0, 0.0).unwrap();

        js_sleep(clock).await.unwrap();
        console_log!("Hello world {}", i);

        i += 1;
    }


  /*   


    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    context.rect(0.0, 0.0, 100.0, 100.0);
    context.stroke();

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();

    set_timeout(&window, &closure, 2000); */
}