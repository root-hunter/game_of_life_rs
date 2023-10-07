use wasm_bindgen::prelude::*;
use web_sys::{ImageData, CanvasRenderingContext2d};
use image::{RgbaImage, Rgba};

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
    
const SIZE: u32 = 4096;
const USIZE: usize = SIZE as usize;
const FPS: u32 = 300;
const FPR_REFRESH_MS: f64 = 300.0;
const CELL_FOR_SIDE: u32 = SIZE/4;
const CELL_FOR_SIDE_USIZE: usize = CELL_FOR_SIDE as usize;
const START_CELL: usize = USIZE*8;

const CELL_PIXEL_SIZE: u32 = SIZE/CELL_FOR_SIDE;
const CLOCK: u32 = 1000/FPS;

const MATRIX_INDEX_CHECKS: [[i32; 2]; 8] = [
    [-1, -1], [-1, 0],
    [-1, 1], [0, -1],
    [0, 1], [1, -1],
    [1, 0], [1, 1]
];

type MatrixType = [[u8; CELL_FOR_SIDE_USIZE]; CELL_FOR_SIDE_USIZE];


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
    for x in (i*CELL_PIXEL_SIZE)..((i + 1)*(CELL_PIXEL_SIZE)){
        for y in (j*CELL_PIXEL_SIZE)..((j + 1)*(CELL_PIXEL_SIZE)){
            img.put_pixel(x, y, Rgba([0, 0, 0, 255]));
        }
    }
}

fn update_canvas(context: &CanvasRenderingContext2d, imagedata: &ImageData){
    context.put_image_data(imagedata, 0.0, 0.0).unwrap();
}

fn reset_all(image: &mut RgbaImage, context: &CanvasRenderingContext2d, imagedata: &ImageData){
    let r_random = (random() * 255.0) as u8;
    let g_random = (random() * 255.0) as u8;
    let b_random = (random() * 255.0) as u8;

    draw_background(image, r_random, g_random, b_random, 255);
    draw_grid(image, SIZE/CELL_FOR_SIDE);
    update_canvas(&context, &imagedata);
}

fn draw_matrix(image: &mut RgbaImage, matrix: &MatrixType){
    for i in 0..CELL_FOR_SIDE_USIZE{
        for j in 0..CELL_FOR_SIDE_USIZE{
            if matrix[i][j] == 1 {
                fill_cell(image, i as u32, j as u32);
            }
        }
    }
}

fn matrix_random_fill(image: &mut RgbaImage, matrix: &mut MatrixType, max: usize){
    let mut randx = (random()*CELL_FOR_SIDE as f64) as usize;
    let mut randy = (random()*CELL_FOR_SIDE as f64) as usize;

    for _ in 0..max {
        randx = (random()*CELL_FOR_SIDE as f64) as usize;
        randy = (random()*CELL_FOR_SIDE as f64) as usize;

        matrix[randx][randy] = 1;
    }

    draw_matrix(image, matrix);
}

fn matrix_count_alive(matrix: &MatrixType) -> u32 {
    return matrix.iter().flatten().sum::<u8>() as u32;
}

fn cell_count_neighbors(matrix: &mut MatrixType, total_alive: &mut u32) {
    for i in 0..CELL_FOR_SIDE_USIZE{
        for j in 0..CELL_FOR_SIDE_USIZE{
            let mut count: u32 = 0;
            let is_alive = matrix[i][j] == 1;

            for k in 0..MATRIX_INDEX_CHECKS.len(){
                let x = (i as i32 + MATRIX_INDEX_CHECKS[k][0]) as i32;
                let y = (j as i32 + MATRIX_INDEX_CHECKS[k][1]) as i32;

                if x < CELL_FOR_SIDE_USIZE as i32 && x >= 0 
                && y < CELL_FOR_SIDE_USIZE as i32 && y >= 0{

                   if matrix[x as usize][y as usize] == 1 {
                        count += 1;
                    }

                    if count >= 4{
                        break;
                    }
                }
            }

            if is_alive && count < 2 {
                matrix[i][j] = 0;
                *total_alive -= 1;
            }else if is_alive && count > 3 {
                matrix[i][j] = 0;
                *total_alive -= 1;
            }else if !is_alive && count == 3 {
                matrix[i][j] = 1;
                *total_alive += 1;
            }
        }
    }
}


#[wasm_bindgen(start)]
async fn start() {
    let mut total_alive: u32 = START_CELL.try_into().unwrap();
    let mut matrix: MatrixType = [[0; CELL_FOR_SIDE_USIZE]; CELL_FOR_SIDE_USIZE];

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
    //console_log!("{:?}", image_raw);
    
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

    let test = js_sys::Function::new_no_args("
        console.log('Ciaosoosososo');
    ");

    canvas.set_onclick(Option::from(&test));

    reset_all(&mut image, &context, &imagedata);

    let mut t1 = Date::now();

    let mut temp = Date::now();
    let mut delta = Math::abs(temp - Date::now());
    let mut fps_calc = FPS as f64 + ((1000.0 - (delta*FPS as f64))*FPS as f64)/1000.0;
    let mut fps_string = format!("FPS: {}", fps_calc);
    let mut now = Date::now();

    matrix_random_fill(&mut image, &mut matrix, START_CELL);

    loop {
        let total_alive_cells = matrix_count_alive(&matrix);

        console_log!("{}", total_alive);

        if total_alive_cells < (CELL_FOR_SIDE*CELL_FOR_SIDE) - 1 {
            temp = Date::now();
            cell_count_neighbors(&mut matrix, &mut total_alive);
            draw_matrix(&mut image, &matrix);
            update_canvas(&context, &imagedata);
            js_sleep(CLOCK as i32).await.unwrap();
        
            now = Date::now();
            delta = Math::abs(temp - now);
    
            if temp - now >= FPR_REFRESH_MS {
                t1 = now;
                fps_calc = FPS as f64 + ((1000.0 - (delta*FPS as f64))*FPS as f64)/1000.0;
                fps_string = format!("FPS: {}", fps_calc);
                fps_text.set_text_content(Option::from(fps_string.as_str()));
            }
        }else{
            console_log!("Virus are win");
            break;
        }
    }
}