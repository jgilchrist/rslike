use std::vec::Vec;

use engine::Actor;
use util::Point;

pub struct World {
    player: Actor,
    actors: Vec<Actor>,
}

impl World {

    pub fn new() -> World {
        World {
            player: Actor::new("Player".to_string(), Point::new(1, 1), 100, '@'),
            actors: Vec::new(),
        }
    }
    
}
