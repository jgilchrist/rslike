use util::units::{Direction, Point};

pub struct Actor {
    pub name: String,
    pub pos: Point,
    pub health: i32,
    pub max_health: i32,
    pub repr: char,
}

impl Actor {

    pub fn new(name: &'static str, pos: Point, max_health: i32, repr: char) -> Actor {
        Actor {
            name: name.to_string(),
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

    pub fn hurt(&mut self, amount: i32) {
        self.health -= amount;
    }

    pub fn heal(&mut self, amount: i32) {
        self.health += amount;
    }

    pub fn kill(&mut self) {
        self.health = 0;
    }

}
