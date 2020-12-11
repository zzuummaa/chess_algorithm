#![allow(dead_code)]

pub struct Position {
    x: u8,
    y: u8
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}