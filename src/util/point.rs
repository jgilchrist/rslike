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

}
