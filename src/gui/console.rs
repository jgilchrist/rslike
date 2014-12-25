use util::units::{Point, Size};

pub trait Console {
    fn put(&mut self, Point, char);
    fn clear(&mut self);
    fn flush(&mut self);

    fn check_for_keypress(&mut self) -> Option<Key>;

    fn size(&self) -> Size;
    fn window_closed(&self) -> bool;
}

#[deriving(Copy, PartialEq, Show)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Escape,
}
