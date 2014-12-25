use engine::Tile;
use util::units::{AsTuple, Size, Point};

use std::iter::repeat;

pub struct Map {
    tiles: Vec<Vec<Tile>>,
    pub size: Size,
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

    pub fn at(&self, loc: Point) -> Tile {
        self.tiles[loc.y as uint][loc.x as uint]
    }

    pub fn is_walkable(&self, loc: Point) -> bool {
        self.at(loc).walkable()
    }

    pub fn set_tile(&mut self, loc: Point, tile: Tile) {
        self.tiles[loc.y as uint][loc.x as uint] = tile;
    }

}
