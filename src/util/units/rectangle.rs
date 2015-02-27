use util::units::{Point, Size};

pub struct Rectangle {
    location: Point,
    size: Size,
}

impl Rectangle {

    pub fn new(location: Point, size: Size) -> Rectangle {
        Rectangle { location: location, size: size }
    }

    pub fn move_dir(&self, location: Point) -> Rectangle {
        Rectangle { location: self.location + location, size: self.size }
    }

    pub fn resize(&self, size: Size) -> Rectangle {
        Rectangle { location: self.location, size: self.size + size }
    }

    pub fn location(&self) -> Point {
        return self.location;
    }

    pub fn width(&self) -> i32 {
        return self.size.x;
    }

    pub fn height(&self) -> i32 {
        return self.size.y;
    }

}
