use engine::{IntoMap, Tile};
use util::units::{Point, Size};

use std::path::AsPath;
use std::fs::File;
use std::io::Read;

/// A map.
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub size: Size,
}

impl Map {

    /// Creates a new map.
    ///
    /// This method takes a type which implements the `IntoMap` trait to create
    /// the new map.
    ///
    /// # Examples
    ///
    /// From a 2D vector of Tiles:
    ///
    /// ```
    /// use rslike::engine::Map;
    ///
    /// let tiles = vec![vec![]];
    /// let map = Map::new(tiles);
    /// ```
    ///
    /// From a String:
    ///
    /// ```
    /// use rslike::engine::Map;
    ///
    /// let mapstr = "###\n# #\n###".to_string();
    /// let map = Map::new(mapstr);
    /// ```
    pub fn new<T>(maplike: T) -> Map where T: IntoMap {
        maplike.as_map().unwrap()
    }

    pub fn from_file<T>(path: T) -> Map where T: AsPath {
        let mut level_file = File::open(path).ok().expect("Could not find level file");

        let mut level_string = String::new();
        level_file.read_to_string(&mut level_string).ok().expect("Could not read from level file");

        Map::new(level_string)
    }

    pub fn at(&self, loc: Point) -> Tile {
        self.tiles[loc.y as usize][loc.x as usize]
    }

    pub fn is_walkable(&self, loc: Point) -> bool {
        self.at(loc).is_walkable()
    }

    pub fn set_tile(&mut self, loc: Point, tile: Tile) {
        self.tiles[loc.y as usize][loc.x as usize] = tile;
    }

    pub fn height(&self) -> i32 {
        self.tiles.len() as i32
    }

    pub fn width(&self) -> i32 {
        self.tiles[0].len() as i32
    }

}
