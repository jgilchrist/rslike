use engine::{MessageList, World};

/// The entire game state.
pub struct Game {
    pub world: World,
    pub log: MessageList,
}

impl Game {

    /// Creates a new game.
    pub fn new() -> Game {
        Game {
            world: World::new(),
            log: MessageList::new(),
        }
    }

    pub fn step(&mut self) {

    }

}
