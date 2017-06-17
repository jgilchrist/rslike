use util::units::AsTuple;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}

impl Offset {
    pub fn new(x: i32, y: i32) -> Offset {
        Offset { x: x, y: y }
    }

    pub fn zero() -> Offset {
        Offset { x: 0, y: 0 }
    }
}

impl AsTuple<i32> for Offset {
    fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl AsTuple<usize> for Offset {
    fn as_tuple(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}
