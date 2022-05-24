use crate::common::{Point, Size};
use std::collections::HashSet;

pub enum NodeType {
    Free,
    Wall,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Maze {
    pub size: Size,
    pub points: SparsePointSet,
    pub source: Point,
    pub destination: Point
}

impl Maze {
    pub fn new(size: Size, points: SparsePointSet, source: Point, destination: Point) -> Maze {
        Maze { size, points, source, destination }
    }
}
