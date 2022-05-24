use crate::common::{Point, Size};
use std::collections::HashSet;

pub enum NodeType {
    Free,
    Wall,
}

pub struct SparsePointSet {
    walls: HashSet<Point>,
}

impl SparsePointSet {
    pub fn new<PointsIter: Iterator<Item = Point>>(walls: PointsIter) -> Self {
        SparsePointSet {
            walls: walls.collect(),
        }
    }

    pub fn get(&self, coord: &Point) -> NodeType {
        if self.walls.contains(coord) {
            NodeType::Wall
        } else {
            NodeType::Free
        }
    }
}

pub struct Maze {
    pub size: Size,
    pub points: SparsePointSet,
}

impl Maze {
    pub fn new(size: Size, points: SparsePointSet) -> Maze {
        Maze { size, points }
    }
}
