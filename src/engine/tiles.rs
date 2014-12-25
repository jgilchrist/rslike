use std::fmt;

#[deriving(Clone, Copy)]
pub enum Tile {
    Empty,
    Wall,
    Floor,
}

impl Tile {

    pub fn walkable(&self) -> bool {
        match *self {
            Tile::Empty => true,
            Tile::Wall => false,
            Tile::Floor => true,
        }
    }

    pub fn repr(&self) -> char {
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
