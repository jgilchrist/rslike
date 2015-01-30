use util::units::{AsTuple, Direction};

use std::ops::Add;

#[derive(Copy, Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {

    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    pub fn zero() -> Point {
        Point { x: 0, y: 0 }
    }

    pub fn move_dir(&self, dir: Direction) -> Point {
        *self + dir.as_tuple()
    }

}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        let (ox, oy) = other.as_tuple();
        Point { x: self.x + ox, y: self.y + oy }
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Point;

    fn add(self, other: (i32, i32)) -> Point {
        let (ox, oy) = other;
        Point { x: self.x + ox, y: self.y + oy }
    }
}

impl AsTuple<i32> for Point {
    fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
