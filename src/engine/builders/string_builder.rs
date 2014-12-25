use engine::{Map, Tile};
use engine::builders::MapBuilder;
use util::units::{AsTuple, Size};

use std::iter::repeat;

pub struct MapFromString {
    s: String,
}

impl MapFromString {

    pub fn new(s: &str) -> MapFromString {
        MapFromString { s: s.to_string() }
    }

}

impl MapBuilder for MapFromString {
    fn build(&self) -> Map {
        let size = Size::new(10, 10);
        let (width, height) = size.as_tuple();
        let tiles: Vec<Vec<Tile>> = range(0, height).map(|_| repeat(Tile::Empty).take(width).collect()).collect();
        Map::new(tiles)
    }
}
