#![allow(dead_code)]

use std::fmt;
use std::fmt::{Display, Formatter};

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Rank {
    NONE,
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN,
}

pub const W_PAWN: i32 = 1000;
pub const W_BISHOP: i32 = 3 * W_PAWN;
pub const W_KNIGHT: i32 = W_BISHOP;
pub const W_ROOK: i32 = 5 * W_PAWN;
pub const W_QUEEN: i32 = 9 * W_PAWN;
pub const W_INFINITY: i32 = 10 * W_QUEEN;
pub const W_KING: i32 = W_INFINITY;

const FIGURE_WEIGHT: [i32; 7] = [0, W_KING, W_QUEEN, W_ROOK, W_BISHOP, W_KNIGHT, W_PAWN];

impl From<u8> for Rank {
    fn from(item: u8) -> Self {
        unsafe { return ::std::mem::transmute(item) };
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    NONE = 0,
    WHITE = 64,
    BLACK = 128,
}

impl From<u8> for Color {
    fn from(item: u8) -> Self {
        unsafe { return ::std::mem::transmute(item) };
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Figure(u8);

impl Figure {
    pub fn new(rank: Rank, color: Color) -> Figure {
        Figure {
            0: (rank as u8 + color as u8),
        }
    }

    pub fn rank(&self) -> Rank {
        Rank::from(self.0 & 7)
    }

    pub fn color(&self) -> Color {
        Color::from(self.0 & (64 + 128))
    }

    pub fn weight(&self) -> i32 {
        FIGURE_WEIGHT[self.rank() as usize]
    }
}

impl Display for Figure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let color = match self.color() {
            Color::NONE => 'n',
            Color::WHITE => 'w',
            Color::BLACK => 'b',
        };
        let rank = match self.rank() {
            Rank::NONE => 'n',
            Rank::KING => 'K',
            Rank::QUEEN => 'Q',
            Rank::ROOK => 'r',
            Rank::BISHOP => 'b',
            Rank::KNIGHT => 'k',
            Rank::PAWN => 'p',
        };
        write!(f, "{}{}", color, rank)
    }
}
