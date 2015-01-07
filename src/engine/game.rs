use engine::World;
use util::units::{Direction};

pub struct Game {
    pub world: World,
}

impl Game {

    pub fn new() -> Game {
        Game {
            world: World::new(),
        }
    }

    pub fn step(&mut self) {

    }

    pub fn walk(&mut self, direction: Direction) {
        self.world.walk(direction);
    }

}
