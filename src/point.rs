use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::Add;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Point {
    x: i8,
    y: i8,
}

impl Point {
    pub fn new(x: i8, y: i8) -> Point {
        Point { x, y }
    }

    pub fn x(&self) -> i8 {
        self.x
    }

    pub fn y(&self) -> i8 {
        self.y
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", ('H' as i8 - self.x) as u8 as char)?;
        write!(f, "{}", ('1' as i8 + self.y) as u8 as char)
    }
}