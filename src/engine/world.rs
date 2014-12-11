use std::vec::Vec;

use engine::Actor;
use util::units::{Direction, Point};

pub struct World {
    pub player: Actor,
    pub actors: Vec<Actor>,
}

impl World {

    pub fn new() -> World {
        World {
            player: Actor::new("Player".to_string(), Point::new(1, 1), 100, '@'),
            actors: Vec::new(),
        }
    }

    pub fn walk(&mut self, direction: Direction) {
        self.player.walk(direction);
    }
    
}
