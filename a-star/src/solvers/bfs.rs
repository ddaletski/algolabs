use std::collections::HashSet;

use crate::common::{Point, Rect};
use crate::maze::{Maze, NodeType};
use crate::traits::solver::{CellState, Progress, SearchState};
pub use crate::traits::Solver;

pub struct BFSSolver {
    maze: Maze,
    checked: HashSet<Point>,
    queue: std::collections::LinkedList<Point>,
    found: bool,
}

impl Solver for BFSSolver {
    fn inspect(&self) -> Vec<u8> {
        let size = &self.maze.size;
        let mut grid: Vec<u8> = Vec::new();
        grid.resize(size.width as usize * size.height as usize, 0 as u8);

        let queued: HashSet<Point> = self.queue.clone().into_iter().collect();

        for y in 0..size.height as usize {
            let skip = y * size.width as usize;
            let row = &mut grid[skip..];
            for x in 0..size.width as usize {
                let point = Point {
                    x: x as i32,
                    y: y as i32,
                };

                row[x] = if point == self.maze().source {
                    CellState::Source as u8
                } else if point == self.maze().destination {
                    CellState::Destination as u8
                } else if self.checked.contains(&point) {
                    CellState::Checked as u8
                } else if queued.contains(&point) {
                    CellState::Queued as u8
                } else {
                    self.maze.points.get(&point) as u8
                };
            }
        }

        grid
    }

    fn next(&mut self) -> SearchState {
        if self.found {
            return SearchState::Found;
        } else if self.queue.is_empty() {
            return SearchState::NotFound;
        }

        let current = self.queue.pop_front().unwrap();

        if current == self.maze().destination {
            self.found = true;
            self.queue.clear();
            return SearchState::Found;
        } else if !self.checked.contains(&current) {
            self.checked.insert(current);

            let left = current.shift(-1, 0);
            let right = current.shift(1, 0);
            let down = current.shift(0, 1);
            let up = current.shift(0, -1);

            let candidates = [left, right, down, up];

            for candidate in candidates {
                self.add_candidate(candidate);
            }
        }

        SearchState::Progress(Progress {
            in_queue: self.queue.len() as u32,
            checked: self.checked.len() as u32,
        })
    }

    fn restart(&mut self) {
        self.queue.clear();
        self.checked.clear();
        self.found = false;

        self.add_candidate(self.maze().source);
    }

    fn maze(&self) -> &Maze {
        &self.maze
    }
}

impl BFSSolver {
    pub fn new(maze: Maze) -> Self {
        let mut solver = Self {
            maze,
            checked: HashSet::new(),
            queue: std::collections::LinkedList::new(),
            found: false,
        };

        solver.restart();

        solver
    }

    fn add_candidate(&mut self, point: Point) {
        // skip cached
        if self.checked.contains(&point) {
            return;
        }

        // skip out of maze bounds points
        let bounds = Rect::of_size(self.maze.size);
        if !bounds.contains(&point) {
            return;
        }

        // skip walls
        if let NodeType::Wall = self.maze.points.get(&point) {
            return;
        };

        self.queue.push_back(point);
    }
}
