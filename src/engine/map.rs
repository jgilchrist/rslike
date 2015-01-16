use engine::{IntoMap, Tile};
use util::units::{Point, Size};

pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub size: Size,
}

impl Map {

    pub fn new<T>(mappish: T) -> Map where T: IntoMap {
        let result = mappish.into_map();

        let map = match result {
            Ok(map) => map,
            Err(msg) => {
                panic!(msg);
            }
        };

        map
    }

    pub fn at(&self, loc: Point) -> Tile {
        self.tiles[loc.y as usize][loc.x as usize]
    }

    pub fn is_walkable(&self, loc: Point) -> bool {
        self.at(loc).walkable()
    }

    pub fn set_tile(&mut self, loc: Point, tile: Tile) {
        self.tiles[loc.y as usize][loc.x as usize] = tile;
    }

}
