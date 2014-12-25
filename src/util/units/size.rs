use util::units::AsTuple;

#[deriving(Show, Copy)]
pub struct Size {
    x: uint,
    y: uint
}

impl Size {

    pub fn new(x: uint, y: uint) -> Size {
        Size { x: x, y: y }
    }

}

impl AsTuple<uint> for Size {
    fn as_tuple(&self) -> (uint, uint) {
        (self.x, self.y)
    }
}
