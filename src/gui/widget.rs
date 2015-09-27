use gui::{Color, Console};
use util::units::{BorderedRectangle, Point, Size};

use tcod;

pub struct Widget {
    pub rect: BorderedRectangle,
}

impl Widget {
    pub fn new(location: Point, size: Size) -> Widget {
        Widget { rect: BorderedRectangle::new(location, size) }
    }

    pub fn put_plain(&self, console: &mut Console, pos: Point, c: char) {
        console.put_plain(self.adjusted_position(pos), c);
    }

    pub fn put(&self, console: &mut Console, pos: Point, c: char, f_color: Color, b_color: Color) {
        console.put(self.adjusted_position(pos), c, f_color, b_color);
    }

    pub fn print_plain(&self, console: &mut Console, pos: Point, text: &str) {
        console.print_plain(self.adjusted_position(pos), text);
    }

    pub fn print(&self, console: &mut Console, pos: Point, text: &str, f_color: Color, b_color: Color) {
        console.print(self.adjusted_position(pos), text, f_color, b_color);
    }

    pub fn print_align(&self, console: &mut Console, pos: Point, text: &str, alignment: tcod::TextAlignment) {
        console.print_align(self.adjusted_position(pos), text, alignment);
    }

    // TODO: How to clear a subset of the console?
    // pub fn clear(&mut self) {
        // self.console.clear();
    // }

    fn adjusted_position(&self, pos: Point) -> Point {
        // TODO: Error if the position is not in the rectangle
        return self.rect.inner_location() + pos;
    }
}
