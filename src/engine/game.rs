use engine::{MessageList, World};

pub struct Game {
    pub world: World,
    pub log: MessageList,
}

impl Game {

    pub fn new() -> Game {
        Game {
            world: World::new(),
            log: MessageList::new(),
        }
    }

    pub fn step(&mut self) {

    }

}
