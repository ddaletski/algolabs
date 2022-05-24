use std::hash::Hash;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Point {
    pub fn distance_l1(&self, other: &Point) -> f32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as f32
    }

    pub fn distance_l2(&self, other: &Point) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;

        return f32::sqrt(dx * dx + dy * dy);
    }

    pub fn shift(&self, x: i32, y: i32) -> Point {
        return Point {
            x: self.x + x,
            y: self.y + y,
        };
    }
}

impl Size {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rect {
    pub fn of_size(size: Size) -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: size.width as u32,
            height: size.height as u32,
        }
    }

    pub fn tl(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    pub fn tr(&self) -> Point {
        Point {
            x: self.x + self.width as i32 - 1,
            y: self.y,
        }
    }

    pub fn bl(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + self.height as i32 - 1,
        }
    }

    pub fn br(&self) -> Point {
        Point {
            x: self.x + self.width as i32 - 1,
            y: self.y + self.height as i32 - 1,
        }
    }

    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.x
            && point.y >= self.y
            && point.x < self.x + self.width as i32
            && point.y < self.y + self.height as i32
    }
}
