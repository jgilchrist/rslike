use engine::{MessageList, World};
use util::units::Direction;

/// The entire game state.
pub struct Game {
    pub world: World,
    pub log: MessageList,
}

pub enum Command {
    Walk(Direction),
}

impl Game {

    /// Creates a new game.
    pub fn new() -> Game {
        Game {
            world: World::new(),
            log: MessageList::new(),
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
