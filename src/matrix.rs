use crate::constants::*;
use crate::draw::*;
use crate::types::*;
use image::RgbaImage;

use js_sys::Math::random;
use rayon::prelude::IntoParallelRefIterator;
use rayon::prelude::ParallelIterator;

pub fn draw_matrix(image: &mut RgbaImage, matrix: &MatrixArcType){
    for i in 0..CFSU {
        for j in 0..CFSU{
            if matrix[i * CFSU + j] == 1 {
                fill_cell(image, i as u32, j as u32);
            }
        }
    }
}

pub fn matrix_random_fill(image: &mut RgbaImage, matrix: &mut MatrixArcType, matrix_cnt: &mut MatrixArcType, max: usize){
    let mut randx: usize;
    let mut randy: usize;

    for _ in 0..max {
        randx = (random()*CELL_FOR_SIDE as f64) as usize;
        randy = (random()*CELL_FOR_SIDE as f64) as usize;

        matrix[randx * CFSU + randy] = 1;
        matrix_cnt[randx * CFSU + randy] = 1;
    }

    draw_matrix(image, matrix);
}

pub fn matrix_count(matrix: &MatrixArcType) -> u32{
    return matrix.par_iter().map(|x| *x as u32).sum();
}

pub fn cell_count_neighbors(matrix: &mut MatrixArcType, matrix_count: &mut MatrixArcType) {
    let mut count;
    let mut is_alive;
    let mut x: i32;
    let mut y: i32;

    for i in 0..CFSU{
        for j in 0..CFSU {
            count = 0;
            is_alive = matrix[i * CFSU + j] == 1;

            let mut k: usize = 0;
            while k < 8 && count < 4 {
                x = (i as i32 + MATRIX_INDEX_CHECKS[k][0] as i32) as i32;
                y = (j as i32 + MATRIX_INDEX_CHECKS[k][1] as i32) as i32;

                if (x >= 0 && x < CFSU as i32)  
                    && (y >= 0 && y < CFSU as i32) {

                    let x = x as usize;
                    let y = y as usize;

                   if matrix[x * CFSU + y] == 1 {
                        count += 1;
                    }
                }

                k += 1;    
            }

            if is_alive && (count < 2 || count > 3) {
                matrix[i * CFSU + j] = 0;
            }else if !is_alive && count == 3 {
                matrix[i * CFSU + j] = 1;
                if matrix_count[i * CFSU + j] == 0{
                    matrix_count[i * CFSU + j] = 1;
                }
            }
        }
    }
}


