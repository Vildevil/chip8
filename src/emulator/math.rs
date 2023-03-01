pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point {x, y }
    }
}


pub fn point(x: usize, y: usize) -> Point {
    Point::new(x, y)
}