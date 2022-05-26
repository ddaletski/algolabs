use std::collections::{BinaryHeap, HashSet};

use crate::common::{Point, Rect};
use crate::maze::{Maze, NodeType};
use crate::traits::solver::{CellState, Progress, SearchState};
pub use crate::traits::Solver;

struct QueueEntry {
    cost: f32,
    point: Point,
    destination: Point,
    index_number: u32,
}

impl QueueEntry {
    pub fn priority(&self) -> i32 {
        let dist_func = Point::distance_l1;

        let remaining_estimate = dist_func(&self.point, &self.destination);
        let current_cost = self.cost;
        // let recentness_discount = (0.1 * self.index_number as f32) as i32;
        -(current_cost + remaining_estimate) as i32
    }
}

impl PartialEq for QueueEntry {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}
impl Eq for QueueEntry {}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        i32::partial_cmp(&self.priority(), &other.priority())
    }
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        i32::cmp(&self.priority(), &other.priority())
    }
}

pub struct AStarSolver {
    maze: Maze,
    checked: HashSet<Point>,
    queue: BinaryHeap<QueueEntry>,
    found: bool,
    max_index: u32,
}

impl Solver for AStarSolver {
    fn inspect(&self) -> Vec<u8> {
        let size = &self.maze.size;
        let mut grid: Vec<u8> = Vec::new();
        grid.resize(size.width as usize * size.height as usize, 0 as u8);

        let queued: HashSet<Point> = self.queue.iter().map(|it| it.point).collect();

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

        let QueueEntry {
            cost,
            point,
            destination: _,
            index_number: _,
        } = self.queue.pop().unwrap();

        if point == self.maze().destination {
            self.found = true;
            self.queue.clear();
            return SearchState::Found;
        } else if !self.checked.contains(&point) {
            self.checked.insert(point);

            self.add_candidate(point.shift(1, 0), cost + 1.0);
            self.add_candidate(point.shift(-1, 0), cost + 1.0);
            self.add_candidate(point.shift(0, 1), cost + 1.0);
            self.add_candidate(point.shift(0, -1), cost + 1.0);
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

        self.add_candidate(self.maze().source, 0.0);
    }

    fn maze(&self) -> &Maze {
        &self.maze
    }
}

impl AStarSolver {
    pub fn new(maze: Maze) -> Self {
        let mut solver = Self {
            maze,
            checked: HashSet::new(),
            queue: BinaryHeap::new(),
            found: false,
            max_index: 0,
        };

        solver.restart();

        solver
    }

    fn add_candidate(&mut self, point: Point, current_cost: f32) {
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

        self.queue.push(QueueEntry {
            cost: current_cost,
            point,
            destination: self.maze().destination,
            index_number: self.max_index,
        });
        self.max_index += 1;
    }
}
