#![allow(dead_code)]

use crate::figure::*;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ByteBoard {
    /// First index is letter,
    /// Second index is number
    /// Example: A2 -> cells[0][1]
    cells: [[Figure; 8]; 8],
}

impl ByteBoard {
    pub fn new() -> ByteBoard {
        let mut board = ByteBoard {
            cells: [[Figure::new(Rank::NONE, Color::NONE); 8]; 8],
        };

        for i in 0..8 {
            board.cells[i][1] = Figure::new(Rank::PAWN, Color::WHITE);
            board.cells[i][6] = Figure::new(Rank::PAWN, Color::BLACK);
        }

        board.cells[0][0] = Figure::new(Rank::ROOK, Color::WHITE);
        board.cells[7][0] = Figure::new(Rank::ROOK, Color::WHITE);
        board.cells[0][7] = Figure::new(Rank::ROOK, Color::BLACK);
        board.cells[7][7] = Figure::new(Rank::ROOK, Color::BLACK);

        board.cells[1][0] = Figure::new(Rank::KNIGHT, Color::WHITE);
        board.cells[6][0] = Figure::new(Rank::KNIGHT, Color::WHITE);
        board.cells[1][7] = Figure::new(Rank::KNIGHT, Color::BLACK);
        board.cells[6][7] = Figure::new(Rank::KNIGHT, Color::BLACK);

        board.cells[2][0] = Figure::new(Rank::BISHOP, Color::WHITE);
        board.cells[5][0] = Figure::new(Rank::BISHOP, Color::WHITE);
        board.cells[2][7] = Figure::new(Rank::BISHOP, Color::BLACK);
        board.cells[5][7] = Figure::new(Rank::BISHOP, Color::BLACK);

        board.cells[3][0] = Figure::new(Rank::QUEEN, Color::WHITE);
        board.cells[3][7] = Figure::new(Rank::QUEEN, Color::BLACK);

        board.cells[4][0] = Figure::new(Rank::KING, Color::WHITE);
        board.cells[4][7] = Figure::new(Rank::KING, Color::BLACK);

        board
    }
}

impl Display for ByteBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for n in (0..8).rev() {
            write!(f, "{} ", n + 1)?;
            for l in 0..8 {
                write!(f, "{} ", self.cells[l][n])?;
            }
            write!(f, "\n")?;
        }
        write!(f, "  ")?;
        for l in (0u8..8).rev() {
            write!(f, "{}  ", (l + 65) as char)?;
        }
        Ok(())
    }
}
