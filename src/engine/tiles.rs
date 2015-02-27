use gui::{Color, Colors};

use std::fmt;

#[derive(Clone, Copy)]
pub enum Tile {
    Empty,
    Wall,
    Floor,
}

impl Tile {

    pub fn from_char(c: char) -> Tile {
        match c {
            ' ' => Tile::Empty,
            '.' => Tile::Floor,
            '#' => Tile::Wall,
            _ => panic!("No valid tile for character {}", c)
        }
    }

    pub fn is_walkable(&self) -> bool {
        match *self {
            Tile::Empty => true,
            Tile::Wall => false,
            Tile::Floor => true,
        }
    }

    pub fn b_color(&self) -> Color {
        match *self {
            Tile::Empty => Colors::black,
            Tile::Wall => Colors::darker_grey,
            Tile::Floor => Colors::darkest_sepia,
        }
    }

}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match *self {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Floor => '.',
        };

        write!(f, "{}", repr)
    }
}
