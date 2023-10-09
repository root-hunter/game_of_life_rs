use crate::constants::*;

use image::{Rgba, RgbaImage};
use js_sys::Math::random;
use web_sys::{CanvasRenderingContext2d, ImageData};

pub fn draw_background(img: &mut RgbaImage, r: u8, g: u8, b: u8, a: u8) {
    for x in 0..SIZE {
        for y in 0..SIZE {
            img.put_pixel(x, y, Rgba([r, g, b, a]));
            img.put_pixel(y, x, Rgba([r, g, b, a]));
        }
    }
}

pub fn draw_grid(img: &mut RgbaImage, space: u32) {
    let mut x = 0;

    while x < SIZE {
        for y in 0..SIZE {
            img.put_pixel(x, y, COLOR_GRID);
            img.put_pixel(y, x, COLOR_GRID);
        }
        x += space;
    }
}

pub fn fill_cell(image: &mut RgbaImage, i: u32, j: u32) {
    for x in (i * CELL_PIXEL_SIZE)..((i + 1) * (CELL_PIXEL_SIZE)) {
        for y in (j * CELL_PIXEL_SIZE)..((j + 1) * (CELL_PIXEL_SIZE)) {
            image.put_pixel(x, y, COLOR_CELL);
        }
    }
}

pub fn update_canvas(context: &CanvasRenderingContext2d, imagedata: &ImageData) {
    context.put_image_data(imagedata, 0.0, 0.0).unwrap();
}

pub fn reset_all(image: &mut RgbaImage, context: &CanvasRenderingContext2d, imagedata: &ImageData) {
    let r_random = 255 - (random() * 245.0) as u8;
    let g_random = 255 - (random() * 10.0) as u8;
    let b_random = 255 - (random() * 245.0) as u8;

    draw_background(image, r_random, g_random, b_random, 255);
    draw_grid(image, SIZE / CELL_FOR_SIDE);
    update_canvas(&context, &imagedata);
}
