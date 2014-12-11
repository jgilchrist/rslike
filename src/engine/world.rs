use std::vec::Vec;

use engine::Actor;

pub struct World {
    actors: Vec<Actor>,
}

impl World {

    pub fn new() -> World {
        World {
            actors: Vec::new(),
        }
    }
    
}
