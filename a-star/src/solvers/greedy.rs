use std::collections::HashSet;

use crate::common::{Point, Rect};
use crate::maze::{Maze, NodeType};
use crate::traits::solver::{CellState, Progress, SearchState};
pub use crate::traits::Solver;

pub struct GreedySolver {
    maze: Maze,
    checked: HashSet<Point>,
    stack: Vec<Point>,
    found: bool,
}

impl Solver for GreedySolver {
    fn inspect(&self) -> Vec<u8> {
        let size = &self.maze.size;
        let mut grid: Vec<u8> = Vec::new();
        grid.resize(size.width as usize * size.height as usize, 0 as u8);

        let queued: HashSet<Point> = self.stack.clone().into_iter().collect();

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
        if self.stack.is_empty() {
            if self.found {
                return SearchState::Found;
            } else {
                return SearchState::NotFound;
            }
        }

        let current = self.stack.pop().unwrap();

        if current == self.maze().destination {
            self.found = true;
            self.stack.clear();
            return SearchState::Found;
        } else if !self.checked.contains(&current) {
            self.checked.insert(current);

            let left = current.shift(-1, 0);
            let right = current.shift(1, 0);
            let down = current.shift(0, 1);
            let up = current.shift(0, -1);

            let mut candidates = [left, right, down, up];
            candidates.sort_by_key(|&p| -p.distance_l2(&self.maze().destination) as i32);

            for candidate in candidates {
                self.add_candidate(candidate);
            }
        }

        SearchState::Progress(Progress {
            in_queue: self.stack.len() as u32,
            checked: self.checked.len() as u32,
        })
    }

    fn restart(&mut self) {
        self.stack.clear();
        self.checked.clear();
        self.found = false;

        self.add_candidate(self.maze().source);
    }

    fn maze(&self) -> &Maze {
        &self.maze
    }
}

impl GreedySolver {
    pub fn new(maze: Maze) -> Self {
        let mut solver = Self {
            maze,
            checked: HashSet::new(),
            stack: Vec::new(),
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

        self.stack.push(point);
    }
}
