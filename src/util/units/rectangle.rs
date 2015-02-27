use util::units::{Offset, Point, Size};

pub struct Rectangle {
    location: Point,
    size: Size,
}

impl Rectangle {

    pub fn new(location: Point, size: Size) -> Rectangle {
        Rectangle { location: location, size: size }
    }

    pub fn translate(&self, offset: Offset) -> Rectangle {
        Rectangle { location: self.location + offset, size: self.size }
    }

    pub fn resize(&self, offset: Offset) -> Rectangle {
        Rectangle { location: self.location, size: self.size + offset }
    }

    pub fn location(&self) -> Point {
        return self.location;
    }

    pub fn size(&self) -> Size {
        return self.size;
    }

    pub fn width(&self) -> i32 {
        return self.size.x;
    }

    pub fn height(&self) -> i32 {
        return self.size.y;
    }

}
