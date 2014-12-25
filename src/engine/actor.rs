use util::units::{Direction, Point};

pub struct Actor {
    pub name: String,
    pub pos: Point,
    pub health: int,
    pub max_health: int,
    pub repr: char,
}

impl Actor {

    pub fn new(name: String, pos: Point, max_health: int, repr: char) -> Actor {
        Actor {
            name: name,
            pos: pos,
            health: max_health,
            max_health: max_health,
            repr: repr,
        }
    }

    pub fn walk(&mut self, direction: Direction) {
        self.pos = self.pos.move_dir(direction);
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }

    pub fn hurt(&mut self, amount: int) {
        self.health -= amount;
    }

    pub fn heal(&mut self, amount: int) {
        self.health += amount;
    }

    pub fn kill(&mut self) {
        self.health = 0;
    }

}
