use std::vec::Vec;

use engine::{Actor, Map};
use engine::builders::MapFromFile;
use util::units::{Direction, Point};

pub struct World {
    pub player: Actor,
    pub actors: Vec<Actor>,
    pub map: Map,
}

impl World {

    pub fn new() -> World {
        World {
            player: Actor::new("Player", Point::new(10, 10), 100, '@'),
            actors: Vec::new(),
            map: Map::from_builder(MapFromFile::new(Path::new("/home/jonny/code/rslike/assets/maps/test.map"))),
        }
    }

    pub fn walk(&mut self, direction: Direction) {
        if !self.map.is_walkable(self.player.pos.move_dir(direction)) { return; }

        self.player.walk(direction);
    }
    
}
