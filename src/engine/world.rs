use std::vec::Vec;

pub struct World {
    actors: Vec<int>,
}

impl World {

    pub fn new() -> World {
        World {
            actors: Vec::new()
        }
    }
    
}
