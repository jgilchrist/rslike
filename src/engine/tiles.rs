pub enum Tile {
    EmptyTile,
    WallTile,
    FloorTile,
}

pub trait Walkable {
    fn walkable(&self) -> bool;
}

impl Walkable for Tile {
    fn walkable(&self) -> bool {
        match *self {
            Tile::EmptyTile => false,
            Tile::WallTile => false,
            Tile::FloorTile => true,
        }
    }
}
