#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

impl Point {
    pub fn new() -> Self {
        Point { x: 0, y: 0 }
    }
}