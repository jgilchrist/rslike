use std::vec::Vec;

use engine::{Actor, Map};
use engine::builders::MapFromString;
use util::units::{Direction, Point};

pub struct World {
    pub player: Actor,
    pub actors: Vec<Actor>,
    pub map: Map,
}

impl World {

    pub fn new() -> World {
        World {
            player: Actor::new("Player".to_string(), Point::new(1, 1), 100, '@'),
            actors: Vec::new(),
            map: Map::from_builder(MapFromString::new("")),
        }
    }

    pub fn walk(&mut self, direction: Direction) {
        if !self.map.is_walkable(self.player.pos) { return; }

        self.player.walk(direction);
    }
    
}
