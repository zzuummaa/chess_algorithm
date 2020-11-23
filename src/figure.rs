#![allow(dead_code)]

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Rank {
    NONE,
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN
}

pub const W_PAWN: i32 = 1000;
pub const W_BISHOP: i32 = 3 * W_PAWN;
pub const W_KNIGHT: i32 = W_BISHOP;
pub const W_ROOK: i32 = 5 * W_PAWN;
pub const W_QUEEN: i32 = 9 * W_PAWN;
pub const W_INFINITY: i32 = 10 * W_QUEEN;
pub const W_KING: i32 = W_INFINITY;

const FIGURE_WEIGHT: [i32; 7] = [ 0, W_KING, W_QUEEN, W_ROOK, W_BISHOP, W_KNIGHT, W_PAWN ];

impl From<u32> for Rank {
    fn from(item: u32) -> Self {
        unsafe { return ::std::mem::transmute(item) };
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    WHITE = 64,
    BLACK = 128
}

impl From<u32> for Color {
    fn from(item: u32) -> Self {
        unsafe { return ::std::mem::transmute(item) };
    }
}

pub struct Figure(u32);

pub fn build_figure(rank: Rank, color: Color) -> Figure {
    Figure{ 0: (rank as u32 + color as u32) }
}

impl Figure {
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