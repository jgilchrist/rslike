pub trait XYPair<T> {
    fn as_tuple(&self) -> (T, T);
}

#[deriving(Show, Copy)]
pub struct Point {
    pub x: int,
    pub y: int
}

impl Point {

    pub fn new(x: int, y: int) -> Point {
        Point { x: x, y: y }
    }

    pub fn zero() -> Point {
        Point { x: 0, y: 0 }
    }

    pub fn move_dir(&self, dir: Direction) -> Point {
        *self + dir.as_tuple()
    }

}

impl Add<Point, Point> for Point {
    fn add(self, other: Point) -> Point {
        let (ox, oy) = other.as_tuple();
        Point { x: self.x + ox, y: self.y + oy }
    }
}

impl Add<(int, int), Point> for Point {
    fn add(self, other: (int, int)) -> Point {
        let (ox, oy) = other;
        Point { x: self.x + ox, y: self.y + oy }
    }
}

impl XYPair<int> for Point {
    fn as_tuple(&self) -> (int, int) {
        (self.x, self.y)
    }
}

#[deriving(Show, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl XYPair<int> for Direction {
    fn as_tuple(&self) -> (int, int) {
        match *self {
            Direction::Up      => (  0, -1),
            Direction::Down    => (  0,  1),
            Direction::Left    => ( -1,  0),
            Direction::Right   => (  1,  0),
        }
    }
}

#[deriving(Show, Copy)]
pub struct Size {
    x: uint,
    y: uint
}

impl Size {

    pub fn new(x: uint, y: uint) -> Size {
        Size { x: x, y: y }
    }

}

impl XYPair<uint> for Size {
    fn as_tuple(&self) -> (uint, uint) {
        (self.x, self.y)
    }
}
