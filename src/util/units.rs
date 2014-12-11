use std::fmt;

pub trait XYPair {
    fn as_xy(&self) -> (int, int);
}

pub struct Point {
    x: int,
    y: int
}

impl Point {

    pub fn new(x: int, y: int) -> Point {
        Point { x: x, y: y }
    }

    pub fn zero() -> Point {
        Point { x: 0, y: 0 }
    }

    pub fn move_dir(&self, dir: Direction) -> Point {
        *self + dir.as_xy()
    }

}

impl Add<Point, Point> for Point {
    fn add(&self, other: &Point) -> Point {
        let (ox, oy) = other.as_xy();
        Point { x: self.x + ox, y: self.y + oy }
    }
}

impl Add<(int, int), Point> for Point {
    fn add(&self, other: &(int, int)) -> Point {
        let (ox, oy) = *other;
        Point { x: self.x + ox, y: self.y + oy }
    }
}

impl fmt::Show for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

impl XYPair for Point {
    fn as_xy(&self) -> (int, int) {
        (self.x, self.y)
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl XYPair for Direction {
    fn as_xy(&self) -> (int, int) {
        match *self {
            Direction::Up      => (  0, -1),
            Direction::Down    => (  0,  1),
            Direction::Left    => ( -1,  0),
            Direction::Right   => (  1,  0),
        }
    }
}
