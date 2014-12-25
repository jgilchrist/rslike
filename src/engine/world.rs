use std::vec::Vec;

use engine::{Actor, Map};
use util::units::{Direction, Point, Size};

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
            map: Map::new(Size::new(10, 10)),
        }
    }

    pub fn walk(&mut self, direction: Direction) {
        if !self.map.is_walkable(self.player.pos) { return; }

        self.player.walk(direction);
    }
    
}
