use std::fmt;

pub enum Tile {
    Empty,
    Wall,
    Floor,
}

impl Tile {

    fn walkable(&self) -> bool {
        match *self {
            Tile::Empty => false,
            Tile::Wall => false,
            Tile::Floor => true,
        }
    }

    fn repr(&self) -> char {
        match *self {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Floor => '.',
        }
    }

}

impl fmt::Show for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.repr())
    }
}
