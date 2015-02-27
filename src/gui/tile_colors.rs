use engine::Tile;
use gui::colors::{Color, Colors, Colored};

impl Colored for Tile {

    fn fg_color(&self) -> Color {
        Colors::white
    }

    fn bg_color(&self) -> Color {
        match *self {
            Tile::Empty => Colors::black,
            Tile::Wall => Colors::darker_grey,
            Tile::Floor => Colors::darkest_sepia,
        }
    }

}
