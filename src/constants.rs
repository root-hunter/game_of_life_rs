use image::Rgba;

pub const SIZE: u32 = 4096;
pub const USIZE: usize = SIZE as usize;
pub const FPS: u32 = 400;
pub const CELL_FOR_SIDE: u32 = SIZE / 4;
pub const CFSU: usize = CELL_FOR_SIDE as usize;
pub const START_CELL: usize = USIZE * 10;

pub const CELL_PIXEL_SIZE: u32 = SIZE / CELL_FOR_SIDE;
pub const CLOCK: u32 = 1000 / FPS;

pub const COLOR_GRID: Rgba<u8> = Rgba([0, 0, 0, 255]);
pub const COLOR_CELL: Rgba<u8> = Rgba([0, 0, 0, 255]);

pub const MATRIX_INDEX_SIZE: usize = 8;

pub const MATRIX_INDEX_CHECKS: [[i8; 2]; MATRIX_INDEX_SIZE] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];
