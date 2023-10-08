use crate::constants::*;
use crate::utils::*;

use image::RgbaImage;

use js_sys::Math::random;
use std::sync::{Arc, Mutex, MutexGuard};

pub type MatrixType = [[u8; CELL_FOR_SIDE_USIZE]; CELL_FOR_SIDE_USIZE];
pub type MatrixMutexType = Arc<Mutex<MatrixType>>;
pub type MatrixArcType = Arc<Mutex<MatrixType>>;

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