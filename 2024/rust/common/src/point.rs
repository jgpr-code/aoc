use std::ops::{Add, Mul, Sub};

pub const UP: Point = Point { x: 0, y: -1 };
pub const RIGHT: Point = Point { x: 1, y: 0 };
pub const DOWN: Point = Point { x: 0, y: 1 };
pub const LEFT: Point = Point { x: -1, y: 0 };
pub const NEIGH4: [Point; 4] = [UP, RIGHT, DOWN, LEFT];
pub const UP_RIGHT: Point = Point { x: 1, y: -1 };
pub const DOWN_RIGHT: Point = Point { x: 1, y: 1 };
pub const DOWN_LEFT: Point = Point { x: -1, y: 1 };
pub const UP_LEFT: Point = Point { x: -1, y: -1 };
pub const NEIGH8: [Point; 8] = [
    UP, UP_RIGHT, RIGHT, DOWN_RIGHT, DOWN, DOWN_LEFT, LEFT, UP_LEFT,
];

#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr) => {
        Point::from(&($x, $y))
    };
    ($tuple:expr) => {
        Point::from(&$tuple)
    };
}
pub use crate::point;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i128,
    pub y: i128,
}
impl Point {
    /// check grid bounds (0, 0) <= self < bound
    pub fn inside_point_bound(&self, bound: &Point) -> bool {
        0 <= self.x && self.x < bound.x && 0 <= self.y && self.y < bound.y
    }
    /// returns the 4 neighborhood of the point (like a + sign)
    pub fn get_4_neighbors(&self, bound: &Point) -> Vec<Point> {
        let mut neighbors = Vec::new();
        for delta in NEIGH4 {
            let npoint = *self + &delta;
            if npoint.inside_point_bound(bound) {
                neighbors.push(npoint);
            }
        }
        neighbors
    }
}
impl From<&(i128, i128)> for Point {
    fn from(value: &(i128, i128)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
impl Add<&Point> for Point {
    type Output = Point;
    fn add(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub<&Point> for Point {
    type Output = Point;
    fn sub(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Mul<i128> for Point {
    type Output = Point;
    fn mul(self, rhs: i128) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Mul<&Point> for i128 {
    type Output = Point;
    fn mul(self, rhs: &Point) -> Self::Output {
        Point {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}
