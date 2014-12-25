use engine::Tile;
use util::units::{Size, XYPair};

pub struct Map {
    tiles: Vec<Vec<Tile>>,
    size: Size,
}

impl Map {

    pub fn new(size: Size) -> Map {

        let (width, height) = size.as_tuple();

        // TODO: use range syntax
        let tiles: Vec<Vec<Tile>> = range(0, height).map(|h| range(0, width).map(|w| Tile::Empty).collect()).collect();

        Map {
            tiles: tiles,
            size: size
        }
    }

}
