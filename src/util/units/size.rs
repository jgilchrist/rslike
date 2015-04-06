use util::units::{AsTuple, Offset};

use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    pub x: i32,
    pub y: i32
}

impl Size {

    pub fn new(x: i32, y: i32) -> Size {
        Size { x: x, y: y }
    }


}

impl Add for Size {
    type Output = Size;

    fn add(self, other: Size) -> Size {
        let (ox, oy) : (i32, i32) = other.as_tuple();
        Size { x: self.x + ox, y: self.y + oy }
    }
}

impl Add<Offset> for Size {
    type Output = Size;

    fn add(self, other: Offset) -> Size {
        let (ox, oy) : (i32, i32) = other.as_tuple();
        Size { x: self.x + ox, y: self.y + oy }
    }
}


impl Add<(i32, i32)> for Size {
    type Output = Size;

    fn add(self, other: (i32, i32)) -> Size {
        let (ox, oy) : (i32, i32) = other;
        Size { x: self.x + ox, y: self.y + oy }
    }
}

impl Sub for Size {
    type Output = Size;

    fn sub(self, other: Size) -> Size {
        let (ox, oy) : (i32, i32) = other.as_tuple();
        Size { x: self.x - ox, y: self.y - oy }
    }
}

impl Sub<Offset> for Size {
    type Output = Size;

    fn sub(self, other: Offset) -> Size {
        let (ox, oy) : (i32, i32) = other.as_tuple();
        Size { x: self.x - ox, y: self.y - oy }
    }
}

impl Sub<(i32, i32)> for Size {
    type Output = Size;

    fn sub(self, other: (i32, i32)) -> Size {
        let (ox, oy) = other;
        Size { x: self.x - ox, y: self.y - oy }
    }
}

impl AsTuple<i32> for Size {
    fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl AsTuple<usize> for Size {
    fn as_tuple(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}
