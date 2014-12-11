use util::Point;

pub struct Actor {
    name: String,
    pos: Point,
    health: int,
    max_health: int,
    repr: char,
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

    pub fn hurt(&mut self, amount: int) {
        self.health -= amount;
    }

    pub fn heal(&mut self, amount: int) {
        self.health += amount;
    }

    pub fn kill(&mut self, amount: int) {
        self.health = 0;
    }

}