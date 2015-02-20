use engine::{Actor, Map};
use util::units::{Direction, Point};

use std::path::Path;

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
            map: Map::from_file(Path::new("assets/maps/test.map")),
        }
    }

    pub fn walk(&mut self, direction: Direction) {
        if !self.map.is_walkable(self.player.pos.move_dir(direction)) { return; }

        self.player.walk(direction);
    }

}
