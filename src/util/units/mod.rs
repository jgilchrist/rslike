pub use self::direction::Direction;
pub use self::point::Point;
pub use self::rectangle::Rectangle;
pub use self::size::Size;

mod direction;
mod point;
mod rectangle;
mod size;

pub trait AsTuple<T> {
    fn as_tuple(&self) -> (T, T);
}
