use engine::{Actor, Map};
use util::units::Direction;

pub struct World {
    pub player: Actor,
    pub actors: Vec<Actor>,
    pub map: Map,
}

impl World {

    pub fn new() -> World {
        let map = Map::from_file("assets/maps/test.map");

        World {
            player: Actor::new("Player", map.starting_position, 100),
            actors: Vec::new(),
            map: map,
        }
    }

    pub fn walk(&mut self, direction: Direction) {
        if !self.map.is_walkable(self.player.pos().move_dir(direction)) { return; }

        self.player.walk(direction);
    }

}
