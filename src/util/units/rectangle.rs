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

pub struct BorderedRectangle {
    rect: Rectangle,
}

impl BorderedRectangle {
    pub fn new(location: Point, size: Size) -> BorderedRectangle {
        BorderedRectangle { rect: Rectangle::new(location, size) }
    }

    pub fn translate(&self, offset: Offset) -> BorderedRectangle {
        BorderedRectangle { rect: self.rect.translate(offset) }
    }

    pub fn resize(&self, offset: Offset) -> BorderedRectangle {
        BorderedRectangle { rect: self.rect.resize(offset) }
    }

    pub fn location(&self) -> Point {
        return self.rect.location();
    }

    pub fn inner_location(&self) -> Point {
        return self.location() + Offset::new(1, 1);
    }

    pub fn size(&self) -> Size {
        return self.rect.size();
    }

    pub fn inner_size(&self) -> Size {
        return self.size() - Offset::new(2, 2);
    }

    pub fn width(&self) -> i32 {
        return self.rect.size().x;
    }

    pub fn inner_width(&self) -> i32 {
        return self.width() - 2;
    }

    pub fn height(&self) -> i32 {
        return self.rect.size().y;
    }

    pub fn inner_height(&self) -> i32 {
        return self.height() - 2;
    }
}
