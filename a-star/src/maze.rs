use crate::common::{Point, Size};
use std::collections::HashSet;

pub enum NodeType {
    Free,
    Wall,
}

pub trait PointSet {
    fn get(&self, coord: &Point) -> NodeType;
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
}

impl PointSet for SparsePointSet {
    fn get(&self, coord: &Point) -> NodeType {
        if self.walls.contains(coord) {
            NodeType::Wall
        } else {
            NodeType::Free
        }
    }
}

pub struct Maze {
    pub size: Size,
    pub points: Box<dyn PointSet>,
}

impl Maze {
    pub fn new(size: Size, points_provider: Box<dyn PointSet>) -> Maze {
        Maze {
            size,
            points: points_provider,
        }
    }
}
