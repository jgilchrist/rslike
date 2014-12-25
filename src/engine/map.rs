use engine::Tile;
use util::units::{Size, AsTuple};

use std::iter::repeat;

pub struct Map {
    tiles: Vec<Vec<Tile>>,
    size: Size,
}

impl Map {

    pub fn new(size: Size) -> Map {

        let (width, height) = size.as_tuple();

        // TODO: use range syntax
        let tiles: Vec<Vec<Tile>> = range(0, height).map(|_| repeat(Tile::Empty).take(width).collect()).collect();

        Map {
            tiles: tiles,
            size: size
        }
    }

}
