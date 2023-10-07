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
    
const SIZE: u32 = 2048;
const USIZE: usize = SIZE as usize;
const FPS: u32 = 30;
const FPR_REFRESH_MS: f64 = 60.0;
const CELL_FOR_SIDE: u32 = SIZE/4;
const CELL_FOR_SIDE_USIZE: usize = CELL_FOR_SIDE as usize;
const START_CELL: usize = USIZE*2;

const CELL_PIXEL_SIZE: u32 = SIZE/CELL_FOR_SIDE;
const CLOCK: u32 = 1000/FPS;

const MATRIX_INDEX_CHECKS: [[i32; 2]; 8] = [
    [-1, -1], [-1, 0],
    [-1, 1], [0, -1],
    [0, 1], [1, -1],
    [1, 0], [1, 1]
];

type MatrixType = [[u8; CELL_FOR_SIDE_USIZE]; CELL_FOR_SIDE_USIZE];
type MatrixMutexType = Arc<Mutex<MatrixType>>;

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

const COLOR_GRID: Rgba<u8> = Rgba([0, 255, 30, 255]);
const COLOR_CELL: Rgba<u8> = Rgba([0, 0, 0, 255]);


fn draw_grid(img: &mut RgbaImage, space: u32){
    let mut x = 0;

    while x < SIZE {
        for y in 0..SIZE{
            img.put_pixel(x, y, COLOR_GRID);
            img.put_pixel(y, x, COLOR_GRID);

        }
        x += space;
    }
}

fn fill_cell(image: &mut RgbaImage, i: u32, j: u32){
    for x in (i*CELL_PIXEL_SIZE)..((i + 1)*(CELL_PIXEL_SIZE)){
        for y in (j*CELL_PIXEL_SIZE)..((j + 1)*(CELL_PIXEL_SIZE)){
            image.put_pixel(x, y, COLOR_CELL);
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

fn draw_matrix(image: &mut RgbaImage, mutex: &mut MutexGuard<'_, [[u8; CELL_FOR_SIDE_USIZE]; CELL_FOR_SIDE_USIZE]>){
    for (i, row) in mutex.iter().enumerate(){
        for (j, cell) in row.iter().enumerate() {
            if *cell == 1 {
                fill_cell(image, i as u32, j as u32);
            }
        }
    }
}

fn draw_matrix2(image: &mut RgbaImage,  matrix: &MatrixArcType){
    let mutex = matrix.lock().unwrap();

    for (i, row) in mutex.iter().enumerate(){
        for (j, cell) in row.iter().enumerate() {
            if *cell == 1 {
                fill_cell(image, i as u32, j as u32);
            }
        }
    }
}

fn matrix_random_fill(image: &mut RgbaImage, matrix: &MatrixArcType, max: usize){
    let mutex = &mut matrix.lock().unwrap();

    let mut randx = (random()*CELL_FOR_SIDE as f64) as usize;
    let mut randy = (random()*CELL_FOR_SIDE as f64) as usize;

    for _ in 0..max {
        randx = (random()*CELL_FOR_SIDE as f64) as usize;
        randy = (random()*CELL_FOR_SIDE as f64) as usize;

        mutex[randx][randy] = 1;
    }

    draw_matrix(image, mutex);
}

fn matrix_count_alive(matrix: &MatrixArcType) -> u32 {
    let mutex = &mut matrix.lock().unwrap();

    return mutex.iter().flatten().sum::<u8>() as u32;
}

use rayon::prelude::*;
use std::sync::{Arc, Mutex, MutexGuard};

fn cell_count_neighbors(image: &mut RgbaImage, matrix: &MatrixArcType, total_alive: &mut u32) {
    let mutex = &mut matrix.lock().unwrap();

    for (i, row) in mutex.clone().iter().enumerate(){
        for (j, cell) in row.iter().enumerate() {
            let mut count = 0;
            let is_alive = *cell == 1;

            for k in 0..MATRIX_INDEX_CHECKS.len(){
                let x = (i as i32 + MATRIX_INDEX_CHECKS[k][0]) as i32;
                let y = (j as i32 + MATRIX_INDEX_CHECKS[k][1]) as i32;

                if (x >= 0 && x < CELL_FOR_SIDE_USIZE as i32)  
                    && (y >= 0 && y < CELL_FOR_SIDE_USIZE as i32) {
                    
                   if mutex[x as usize][y as usize] == 1 {
                        count += 1;
                    }

                    if (is_alive && count >= 4) || (!is_alive && count == 3) {
                        break;
                    }
                }
            }

            if is_alive && count < 2 {
                mutex[i][j] = 0;
                //*total_alive -= 1;
            }else if is_alive && count > 3 {
                mutex[i][j] = 0;
                //*total_alive -= 1;
            }else if !is_alive && count == 3 {
                mutex[i][j] = 1;
                //*total_alive += 1;
            }
        }
    }

}

type MatrixArcType = Arc<Mutex<MatrixType>>;

#[wasm_bindgen(start)]
async fn start() {
    let mut total_alive: u32 = START_CELL.try_into().unwrap();
    let matrix: MatrixType = [[0; CELL_FOR_SIDE_USIZE]; CELL_FOR_SIDE_USIZE];
    let mut matrix: Arc<Mutex<MatrixType>> = Arc::new(Mutex::new(matrix));

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let fps_text = document.get_element_by_id("fps").unwrap();
    let fps_text: web_sys::HtmlParagraphElement = fps_text
    .dyn_into::<web_sys::HtmlParagraphElement>()
    .map_err(|_| ())
    .unwrap();


    let epoch_text = document.get_element_by_id("epoch").unwrap();
    let epoch_text: web_sys::HtmlParagraphElement = epoch_text
    .dyn_into::<web_sys::HtmlParagraphElement>()
    .map_err(|_| ())
    .unwrap();



    let alive_text = document.get_element_by_id("alive").unwrap();
    let alive_text: web_sys::HtmlParagraphElement = alive_text
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


    matrix_random_fill(&mut image, &matrix, START_CELL);

    let mut i: usize = 0;
    let mut epoch: usize = 0;
    loop {
        let temp = Date::now();
        let total_alive_cells = matrix_count_alive(&mut matrix);

        cell_count_neighbors(&mut image, &mut matrix, &mut total_alive);
        draw_matrix2(&mut image,  &mut matrix);
        
        update_canvas(&context, &imagedata);
        js_sleep(CLOCK as i32).await.unwrap();
    
        i += 1;
        epoch += 1;

        if i == FPS as usize {
            let now = Date::now();
            let delta = temp - now;
            console_log!("{}", delta);

            let fps_calc = FPS as f64 + ((1000.0 - (delta*FPS as f64))*FPS as f64)/1000.0;
            let fps_string = format!("FPS: {}", fps_calc/60.0);
            fps_text.set_text_content(Option::from(fps_string.as_str()));

            i = 0;

        }

        let epoch_string = format!("EPOCH: {}", epoch);
        epoch_text.set_text_content(Option::from(epoch_string.as_str()));

        let alive_string = format!("ALIVE CELLS: {}", total_alive_cells);
        alive_text.set_text_content(Option::from(alive_string.as_str()));

    
    }
}