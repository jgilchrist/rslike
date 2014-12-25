use engine::Tile;
use engine::builders::MapBuilder;
use util::units::{Point, Size};

pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub size: Size,
}

impl Map {

    pub fn new(tiles: Vec<Vec<Tile>>) -> Map {
        // TODO: check dimensions are the same
        let size = Size::new(tiles.len(), tiles[0].len());

        Map {
            tiles: tiles,
            size: size,
        }
    }

    pub fn from_builder<T>(builder: T) -> Map where T: MapBuilder {
        let result = builder.build();
        match result {
            Ok(map) => map,
            Err(err) => {
                panic!(err.msg);
            }
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
