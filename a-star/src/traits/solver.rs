use crate::maze::Maze;

#[derive(Debug)]
#[repr(u8)]
pub enum CellState {
    Free = 0,
    Wall = 1,
    Checked = 2,
    Queued = 3,
    Source = 4,
    Destination = 5,
}

impl From<u8> for CellState {
    fn from(byte: u8) -> Self {
        match byte {
            0 => CellState::Free,
            1 => CellState::Wall,
            2 => CellState::Checked,
            3 => CellState::Queued,
            4 => CellState::Source,
            5 => CellState::Destination,
            _ => unreachable!("can't convert u8 value '{}' to CellState", byte),
        }
    }
}

pub struct Progress {
    pub in_queue: u32,
    pub checked: u32,
}

pub enum SearchState {
    Progress(Progress),
    Found,
    NotFound,
}

pub trait Solver {
    /// grid data in row-major (C-style) order
    /// every cell u8 value corresponds to CellState value
    fn inspect(&self) -> Vec<u8>;

    fn next(&mut self) -> SearchState;

    fn restart(&mut self);

    fn maze(&self) -> &Maze;
}

