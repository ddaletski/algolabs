pub mod common;
pub mod gui;
pub mod maze;
pub mod solvers;
pub mod traits;

use common::Size;

const MAZE_DIM_SIZE: u32 = 100;

pub const MAZE_SIZE: Size = Size {
    width: MAZE_DIM_SIZE,
    height: MAZE_DIM_SIZE,
};

pub const CELL_SCALE: u32 = 4;
pub const SOLVER_TICKS_PER_SECOND: u64 = 30;
pub const SOLVER_STEPS_PER_TICK: u64 = 10;
