use std::fmt::{Display, Formatter, Error};
use std::fmt;
use std::ops::Add;
use std::hint::unreachable_unchecked;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Point {
    x: i8,
    y: i8,
}

fn sub_char(a: char, b: char) -> i8 {
    a as i8 - b as i8
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

    pub fn from_string(str: &str) -> Result<Self, Error> {
        let mut point = Point::default();
        let parse_count = str.char_indices().filter(|c| {
            match c.0 {
                0..2 => {
                    match c.0 {
                        0 => if c.1 >= 'A' && c.1 <= 'H' { point = point + Point::new(-sub_char(c.1, 'H'), 0) }
                        1 => if c.1 >= '1' && c.1 <= '8' { point = point + Point::new(0, sub_char(c.1, '1')) }
                        _ => unsafe { unreachable_unchecked() }
                    }
                    true
                }
                _ => false
            }
        }).count();

        if parse_count != 2 { return Err(fmt::Error) }

        Ok(point)
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