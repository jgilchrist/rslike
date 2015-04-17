use util::units::{Direction, Point};

/// A single actor in the game
///
/// # Example
///
/// ```
/// use rslike::engine::Actor;
/// use rslike::util::units::Point;
///
/// let actor = Actor::new("Dog", Point::zero(), 100);
/// ```
pub struct Actor {
    name: String,
    pos: Point,
    health: i32,
    max_health: i32,
}

impl Actor {

    /// Creates a new actor
    pub fn new(name: &'static str, pos: Point, max_health: i32) -> Actor {
        Actor {
            name: name.to_string(),
            pos: pos,
            health: max_health,
            max_health: max_health,
        }
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn pos(&self) -> &Point { &self.pos }
    pub fn health(&self) -> i32 { self.health }
    pub fn max_health(&self) -> i32 { self.max_health }

    /// Moves the actor one step in the specified `Direction`
    pub fn walk(&mut self, direction: Direction) {
        self.pos = self.pos.move_dir(direction);
    }

    /// Returns true if the actor's health is equal to or below zero
    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }

    /// Reduces the actor's health by a specified amount
    pub fn hurt(&mut self, amount: i32) {
        self.health -= amount;
    }

    /// Increases the actor's health by a specified amount
    pub fn heal(&mut self, amount: i32) {
        self.health += amount;
    }

    /// Reduces the actor's health to zero
    pub fn kill(&mut self) {
        self.health = 0;
    }

}
