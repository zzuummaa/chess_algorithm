
#[derive(Copy, Clone, Debug, Default)]
pub struct Point {
    x: u8,
    y: u8,
}

impl Point {
    pub fn new(x: u8, y: u8) -> Point {
        Point { x, y }
    }

    pub(crate) fn x(&self) -> u8 {
        self.x
    }

    pub(crate) fn y(&self) -> u8 {
        self.y
    }
}