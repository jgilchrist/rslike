use engine::World;
use engine::log;
use util::units::Direction;

/// The entire game state.
pub struct Game {
    pub world: World,
}

pub enum Command {
    Walk(Direction),
}

impl Game {

    /// Creates a new game.
    pub fn new() -> Game {
        Game {
            world: World::new(),
        }
    }

    pub fn do_command(&mut self, cmd: Command) {
        match cmd {
            Command::Walk(d) => {
                self.world.walk(d);
            }
        }
    }

    pub fn step(&mut self) {

    }

}
