#![allow(dead_code)]

use std::fmt;
use std::fmt::{Display, Formatter};
use crate::figure::Rank::NONE;

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
    OUT
}

pub const W_PAWN: i32 = 1000;
pub const W_BISHOP: i32 = 4 * W_PAWN;
pub const W_KNIGHT: i32 = 3 * W_PAWN;
pub const W_ROOK: i32 = 5 * W_PAWN;
pub const W_QUEEN: i32 = 9 * W_PAWN;
pub const W_INFINITY: i32 = 10 * W_QUEEN;
pub const W_KING: i32 = W_INFINITY;

const FIGURE_WEIGHT: [i32; 8] = [0, W_KING, W_QUEEN, W_ROOK, W_BISHOP, W_KNIGHT, W_PAWN, 0];

impl From<u8> for Rank {
    fn from(item: u8) -> Self {
        unsafe { return ::std::mem::transmute(item & 7) };
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    NONE = 0,
    WHITE = 64,
    BLACK = 128,
    WHITEBLACK = 64 + 128,
}

impl Color {
    pub fn invert(self) -> Self {
        unsafe {
            let c = ::std::mem::transmute::<_, u8>(self) ^ (64 + 128);
            return ::std::mem::transmute(c);
        }
    }
}

impl From<u8> for Color {
    fn from(item: u8) -> Self {
        unsafe { return ::std::mem::transmute(item & (64 + 128)) };
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Figure(u8);

impl Figure {
    pub fn new(rank: Rank, color: Color, flag: bool) -> Figure {
        Figure {
            0: (rank as u8 + color as u8 + ((flag as u8) << 4)),
        }
    }

    pub fn empty() -> Figure {
        Figure::new(NONE, Color::NONE, false)
    }

    pub fn rank(&self) -> Rank {
        Rank::from(self.0)
    }

    pub fn color(&self) -> Color {
        Color::from(self.0)
    }

    pub fn weight(&self) -> i32 {
        FIGURE_WEIGHT[self.rank() as usize]
    }

    pub fn is_flag_set(&self) -> bool {
        (self.0 & 16) == 16
    }

    pub fn set_flag(&self) -> Self {
        let mut changed_self = *self;
        changed_self.0 += 16;
        return changed_self;
    }
}

impl Display for Figure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let color = match self.color() {
            Color::NONE => 'n',
            Color::WHITEBLACK => '%',
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
            Rank::OUT => 'x'
        };
        write!(f, "{}{}", color, rank)
    }
}
