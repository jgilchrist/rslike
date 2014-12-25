use engine::{Map, Tile};
use engine::builders::{MapBuilder, MapBuildResult};
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
    fn build(&self) -> MapBuildResult {
        let lines : Vec<_> = self.s.split('\n').collect();

        println!("{}", lines);

        let size = Size::new(10, 10);
        let (width, height) = size.as_tuple();
        let tiles: Vec<Vec<Tile>> = range(0, height).map(|_| repeat(Tile::Empty).take(width).collect()).collect();
        Ok(Map::new(tiles))
    }
}
