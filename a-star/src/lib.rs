pub mod common;
pub mod gui;
pub mod maze;
pub mod traits;
pub mod solvers;

use common::Size;

const MS: u32 = 100;
pub const MAZE_SIZE: Size = Size {
    width: MS,
    height: MS,
};
pub const CELL_SCALE: u32 = 4;
pub const STEPS_PER_FRAME: u32 = 10;