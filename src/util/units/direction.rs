use util::units::AsTuple;

#[derive(Copy, PartialEq, Show)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl AsTuple<int> for Direction {
    fn as_tuple(&self) -> (int, int) {
        match *self {
            Direction::Up      => (  0, -1),
            Direction::Down    => (  0,  1),
            Direction::Left    => ( -1,  0),
            Direction::Right   => (  1,  0),
        }
    }
}

