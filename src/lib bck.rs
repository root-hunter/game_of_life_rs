use std::f64;
use wasm_bindgen::prelude::*;
use planetarium::ImageFormat;
use planetarium::{Canvas, SpotShape};
use web_sys::ImageData;

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

#[wasm_bindgen(start)]
fn start() {
    
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

    let mut c = Canvas::new(WIDTH, HEIGHT);
    
    // Define a round spot shape with diffraction radius of 2.5 pixels.
    let shape = SpotShape::default().scale(1.0);
    
    // Add some spots at random positions with varying shape size
    // and peak intensity.
    let spot1 = c.add_spot((100.3, 330.8), shape, 0.5);
    let spot2 = c.add_spot((80.6, 200.2), shape.scale(0.5), 0.9);
    
    // Note: Out of range position coordinates and peak intensities are fine.
    //       The resulting spot image is clipped into the canvas rectangle.
    //       Peak intensity > 1.0 leads to saturation to the maximum pixel value.
    let spot3 = c.add_spot((WIDTH as f32/2.0, HEIGHT as f32/2.0), shape.scale(30.0), 1.1);
    
    // Set the canvas background pixel value.
    c.set_background(255);
    
    // Clear the canvas and paint the light spots.
    c.draw();
    
    // Access the rendered image data as a linear pixel array.
    let image_pixbuf = c.pixels();
    console_log!("{:?}", image_pixbuf);
  
    let export_vet_image = c.export_image(ImageFormat::RawGamma8Bpp).unwrap();
    let export_image = export_vet_image.as_slice();

    let data = export_vet_image.as_slice();
    let mut rgba_data = vec![0u8; (WIDTH * HEIGHT * 4) as usize];

    rgba_data.fill(100);

    let k: usize = WIDTH as usize * HEIGHT as usize;
    for i in 0..k {
        rgba_data[i] = data[i];
        //rgba_data[i + k] = data[i];
        //rgba_data[i + k*2] = data[i];
        //rgba_data[i + k*3] = data[i];
    }
    
    let clamped_image = wasm_bindgen::Clamped(rgba_data.as_slice());
    console_log!("ORIGINAL: {:?}", c.export_image(ImageFormat::RawLinear12BppLE).unwrap());

    console_log!("CLAMPED IMAGE LEN: {:?}", data.len());
    console_log!("RGBA IMAGE LEN: {:?}", rgba_data.len());

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

    let closure = wasm_bindgen::prelude::Closure::wrap(Box::new(move |value: JsValue| {
    }) as Box<dyn FnMut(JsValue)>);


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