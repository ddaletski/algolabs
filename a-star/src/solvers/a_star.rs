use std::collections::{BinaryHeap, HashSet};

use crate::common::{Point, Rect};
use crate::maze::{Maze, NodeType};
pub use crate::traits::Solver;
use crate::traits::solver::{CellState, Progress, SearchState};

struct QueueEntry {
    cost: u32,
    point: Point,
}

impl PartialEq for QueueEntry {
    fn eq(&self, other: &Self) -> bool {
        return self.cost == other.cost;
    }
}
impl Eq for QueueEntry {}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        u32::partial_cmp(&self.cost, &other.cost)
    }
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        u32::cmp(&self.cost, &other.cost)
    }
}

pub struct AStarSolver {
    maze: Maze,
    source: Point,
    destination: Point,
    checked: HashSet<Point>,
    queue: BinaryHeap<QueueEntry>,
    found: bool,
}

impl Solver for AStarSolver {
    fn new(maze: Maze, source: Point, destination: Point) -> Self {
        let mut solver = Self {
            maze,
            source,
            destination,
            checked: HashSet::new(),
            queue: BinaryHeap::new(),
            found: false,
        };

        solver.restart();

        solver
    }

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

                row[x] = if point == self.source {
                    CellState::Source as u8
                } else if point == self.destination {
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
        if self.queue.is_empty() {
            if self.found {
                return SearchState::Found;
            } else {
                return SearchState::NotFound;
            }
        }

        let QueueEntry { cost, point } = self.queue.pop().unwrap();

        if point == self.destination {
            self.found = true;
            self.queue.clear();
            return SearchState::Found;
        } else if !self.checked.contains(&point) {
            self.checked.insert(point);

            self.add_candidate(point.shift(1, 0), cost);
            self.add_candidate(point.shift(-1, 0), cost);
            self.add_candidate(point.shift(0, 1), cost);
            self.add_candidate(point.shift(0, -1), cost);
        }

        SearchState::Progress(Progress {
            in_queue: self.queue.len(),
            checked: self.checked.len(),
        })
    }

    fn restart(&mut self) {
        self.queue.clear();
        self.checked.clear();
        self.found = false;

        self.add_candidate(self.source, 0);
    }

    fn maze(&self) -> &Maze {
        &self.maze
    }
}

impl AStarSolver {
    fn cost(&self, point: &Point) -> u32 {
        // self.destination.distance_l1(&point)
        0
    }

    fn add_candidate(&mut self, point: Point, current_cost: u32) {
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
        if let NodeType::Wall = self.maze.points.as_ref().get(&point) {
            return;
        };

        let cost = self.cost(&point) + current_cost;
        self.queue.push(QueueEntry { cost, point });
    }
}
