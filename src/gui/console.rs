use tcod;
use util::units::{Point, Size};
pub use tcod::colors as Colors;

pub trait Console {
    fn put_plain(&mut self, Point, char);
    fn put(&mut self, Point, char, Color, Color);

    fn clear(&mut self);
    fn flush(&mut self);

    fn check_for_keypress(&mut self) -> Option<Key>;

    fn size(&self) -> Size;
    fn window_closed(&self) -> bool;
}

#[derive(Copy, Debug, PartialEq)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Escape,
}

pub type Color = tcod::Color;
