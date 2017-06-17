use util::FromChar;

use std::fmt;

#[derive(Clone, Copy)]
pub enum Tile {
    Empty,
    Wall,
    Floor,
}

impl Tile {
    pub fn is_walkable(&self) -> bool {
        match *self {
            Tile::Empty => true,
            Tile::Wall => false,
            Tile::Floor => true,
        }
    }
}

impl FromChar for Tile {
    type Ret = Tile;

    fn from_char(c: char) -> Tile {
        match c {
            ' ' => Tile::Empty,
            '.' | '@' => Tile::Floor,
            '#' => Tile::Wall,
            _ => panic!("No valid tile for character {}", c),
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
