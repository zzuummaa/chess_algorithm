#![allow(dead_code)]

use std::fmt;
use std::fmt::Formatter;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum FigureRank {
    NONE,
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN
}

impl From<u32> for FigureRank {
    fn from(item: u32) -> Self {
        unsafe { return ::std::mem::transmute(item) };
    }
}

impl fmt::Display for FigureRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

impl fmt::Debug for FigureRank {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

impl PartialEq for FigureRank {
    fn eq(&self, other: &Self) -> bool {
        *self as u32 == *other as u32
    }
}

pub struct Figure(u32);

pub fn build_figure(rank: FigureRank) -> Figure {
    Figure{ 0: rank as u32 }
}

impl Figure {
    pub fn rank(&self) -> FigureRank {
        FigureRank::from(self.0 & 7)
    }

}