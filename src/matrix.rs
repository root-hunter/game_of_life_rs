use crate::constants::*;
use crate::utils::*;
use crate::types::*;
use image::RgbaImage;

use js_sys::Math::random;


pub fn draw_matrix(image: &mut RgbaImage, matrix: &MatrixArcType, total_alive: &mut u32){
    for (i, row) in matrix.iter().enumerate(){
        for (j, cell) in row.iter().enumerate() {
            if *cell == 1 {
                *total_alive += 1;
                fill_cell(image, i as u32, j as u32);
            }
        }
    }
}


pub fn draw_matrix_black(image: &mut RgbaImage){
    for i in 0..CELL_FOR_SIDE_USIZE{
        for j in 0..CELL_FOR_SIDE_USIZE {
            fill_cell(image, i as u32, j as u32);
        }
    }
}


pub fn draw_matrix2(image: &mut RgbaImage,  matrix: &MatrixArcType){
    for (i, row) in matrix.iter().enumerate(){
        for (j, cell) in row.iter().enumerate() {
            if *cell == 1 {
                fill_cell(image, i as u32, j as u32);
            }
        }
    }
}

pub fn matrix_random_fill(image: &mut RgbaImage, matrix: &mut MatrixArcType, max: usize){
    let mut randx = (random()*CELL_FOR_SIDE as f64) as usize;
    let mut randy = (random()*CELL_FOR_SIDE as f64) as usize;

    for _ in 0..max {
        randx = (random()*CELL_FOR_SIDE as f64) as usize;
        randy = (random()*CELL_FOR_SIDE as f64) as usize;

        matrix[randx][randy] = 1;
    }

    let mut t: u32 = 0;
    draw_matrix(image, matrix, &mut t);
}

pub fn matrix_count_alive(matrix: &MatrixArcType) -> u32 {
    return matrix.iter().flatten().sum::<u8>() as u32;
}

pub fn matrix_count(matrix: &MatrixArcType) -> u32{
    let mut s: u32 = 0;
    
    for i in 0..CELL_FOR_SIDE_USIZE{
        for j in 0..CELL_FOR_SIDE_USIZE{
            if matrix[i][j] == 1{
                s += 1;
            }
        }
    }

    return s;
}


pub fn cell_count_neighbors(matrix: &mut MatrixArcType, matrix_count: &mut MatrixArcType, total_alive: &mut u32) {
    let mut count;
    let mut is_alive;
    let mut x: i32;
    let mut y: i32;

    for i in 0..CELL_FOR_SIDE_USIZE{
        for j in 0..CELL_FOR_SIDE_USIZE {
            count = 0;
            is_alive = matrix[i][j] == 1;

            let mut k: usize = 0;
            while k < 8 && count < 4 {
                x = (i as i32 + MATRIX_INDEX_CHECKS[k][0] as i32) as i32;
                y = (j as i32 + MATRIX_INDEX_CHECKS[k][1] as i32) as i32;

                if (x >= 0 && x < CELL_FOR_SIDE_USIZE as i32)  
                    && (y >= 0 && y < CELL_FOR_SIDE_USIZE as i32) {

                   if matrix[x as usize][y as usize] == 1 {
                        count += 1;
                    }
                }

                k += 1;    
            }

            if is_alive && (count < 2 || count > 3) {
                matrix[i][j] = 0;
            }else if !is_alive && count == 3 {
                matrix[i][j] = 1;
            }

            if matrix_count[i][j] == 0 && matrix[i][j] == 1 {
                matrix_count[i][j] = 1;
            }
        }
    }
}


