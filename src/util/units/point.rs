use util::units::{AsTuple, Direction, Offset};

use std::ops::{Add, Sub};

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

    pub fn up(&self, n: i32)    -> Point { *self - (0, n) }
    pub fn down(&self, n: i32)  -> Point { *self + (0, n) }
    pub fn left(&self, n: i32)  -> Point { *self - (n, 0) }
    pub fn right(&self, n: i32) -> Point { *self + (n, 0) }

    pub fn move_dir(&self, dir: Direction) -> Point {
        *self + dir.as_tuple()
    }

}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        let (ox, oy) : (i32, i32) = other.as_tuple();
        Point { x: self.x + ox, y: self.y + oy }
    }
}

impl Add<Offset> for Point {
    type Output = Point;

    fn add(self, other: Offset) -> Point {
        let (ox, oy) : (i32, i32) = other.as_tuple();
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

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        let (ox, oy) : (i32, i32) = other.as_tuple();
        Point { x: self.x - ox, y: self.y - oy }
    }
}

impl Sub<Offset> for Point {
    type Output = Point;

    fn sub(self, other: Offset) -> Point {
        let (ox, oy) : (i32, i32) = other.as_tuple();
        Point { x: self.x - ox, y: self.y - oy }
    }
}

impl Sub<(i32, i32)> for Point {
    type Output = Point;

    fn sub(self, other: (i32, i32)) -> Point {
        let (ox, oy) = other;
        Point { x: self.x - ox, y: self.y - oy }
    }
}

impl AsTuple<i32> for Point {
    fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl AsTuple<usize> for Point {
    fn as_tuple(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}
