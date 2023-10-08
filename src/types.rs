use std::sync::{Mutex, Arc};

use crate::constants::CELL_FOR_SIDE_USIZE;

pub type MatrixType = [[u8; CELL_FOR_SIDE_USIZE]; CELL_FOR_SIDE_USIZE];
pub type MatrixArcType<'a> = [[u8; CELL_FOR_SIDE_USIZE]; CELL_FOR_SIDE_USIZE];