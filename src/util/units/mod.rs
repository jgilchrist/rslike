mod direction;
mod offset;
mod point;
mod rectangle;
mod size;

pub use self::direction::Direction;
pub use self::offset::Offset;
pub use self::point::Point;
pub use self::rectangle::{Rectangle, BorderedRectangle};
pub use self::size::Size;

pub trait AsTuple<T> {
    fn as_tuple(&self) -> (T, T);
}
