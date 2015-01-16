use util::units::AsTuple;

#[derive(Copy, PartialEq, Show)]
pub struct Size {
    pub x: i32,
    pub y: i32
}

impl Size {

    pub fn new(x: i32, y: i32) -> Size {
        Size { x: x, y: y }
    }

}

impl AsTuple<i32> for Size {
    fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
