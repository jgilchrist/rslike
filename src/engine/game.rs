use engine::World;

pub struct Game {
    world: World,
}

impl Game {

    pub fn new() -> Game {
        Game {
            world: World::new(),
        }
    }

    pub fn step(&mut self) {
        
    }

}
