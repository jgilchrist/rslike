use tcod;

pub use tcod::colors as Colors;
pub type Color = tcod::Color;

pub trait Colored {
    fn fg_color(&self) -> Color;
    fn bg_color(&self) -> Color;
}
