#![allow(dead_code)]

use crate::point::*;
use crate::figure::*;
use std::fmt;
use std::fmt::{Display, Formatter, Debug};

#[derive(Debug)]
pub struct ByteBoard {
    /// First index is letter,
    /// Second index is number
    /// Example: A2 -> cells[0][1]
    cells: [[Figure; 16]; 16],
}

impl ByteBoard {
    pub fn new() -> Self {
        let mut board = ByteBoard {
            cells: [[Figure::new(Rank::OUT, Color::NONE); 16]; 16],
        };

        for i in 0..8 {
            for j in 1..7 {
                *board.cell_mut(i, j) = Figure::new(Rank::NONE, Color::NONE);
            }
        }

        for i in 0..8 {
            *board.cell_mut(i, 1) = Figure::new(Rank::PAWN, Color::WHITE);
            *board.cell_mut(i, 6) = Figure::new(Rank::PAWN, Color::BLACK);
        }

        *board.cell_mut(0, 0) = Figure::new(Rank::ROOK, Color::WHITE);
        *board.cell_mut(7, 0) = Figure::new(Rank::ROOK, Color::WHITE);
        *board.cell_mut(0, 7) = Figure::new(Rank::ROOK, Color::BLACK);
        *board.cell_mut(7, 7) = Figure::new(Rank::ROOK, Color::BLACK);

        *board.cell_mut(1, 0) = Figure::new(Rank::KNIGHT, Color::WHITE);
        *board.cell_mut(6, 0) = Figure::new(Rank::KNIGHT, Color::WHITE);
        *board.cell_mut(1, 7) = Figure::new(Rank::KNIGHT, Color::BLACK);
        *board.cell_mut(6, 7) = Figure::new(Rank::KNIGHT, Color::BLACK);

        *board.cell_mut(2, 0) = Figure::new(Rank::BISHOP, Color::WHITE);
        *board.cell_mut(5, 0) = Figure::new(Rank::BISHOP, Color::WHITE);
        *board.cell_mut(2, 7) = Figure::new(Rank::BISHOP, Color::BLACK);
        *board.cell_mut(5, 7) = Figure::new(Rank::BISHOP, Color::BLACK);

        *board.cell_mut(3, 0) = Figure::new(Rank::QUEEN, Color::WHITE);
        *board.cell_mut(3, 7) = Figure::new(Rank::QUEEN, Color::BLACK);

        *board.cell_mut(4, 0) = Figure::new(Rank::KING, Color::WHITE);
        *board.cell_mut(4, 7) = Figure::new(Rank::KING, Color::BLACK);

        board
    }

    pub fn cell_mut(&mut self, literal: usize, number: usize) -> &mut Figure {
        &mut self.cells[literal + 4][number + 4]
    }

    pub fn cell(&self, literal: usize, number: usize) -> &Figure {
        &self.cells[literal + 4][number + 4]
    }

    pub fn point(&self, point: Point) -> &Figure {
        self.cell(point.x() as usize, point.y() as usize)
    }

    pub fn point_mut(&mut self, point: Point) -> &mut Figure {
        self.cell_mut(point.x() as usize, point.y() as usize)
    }

    pub fn cell_iter(&self) -> impl Iterator<Item = (Point, &Figure)> {
        self.cells[4..12].iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row[4..12].iter()
                    .enumerate()
                    .map(move |(y, column)| (Point::new(x as i8, y as i8), column))
            })
    }
}

impl Display for ByteBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (s, e) = (0usize, 8usize);
        for n in (s..e).rev() {
            write!(f, "{} ", n + 1)?;
            for l in s..e {
                write!(f, "{} ", self.cell(l, n))?;
            }
            write!(f, "\n")?;
        }
        write!(f, "  ")?;
        for l in (s..e).rev() {
            write!(f, "{}  ", (l as u8 + 65) as char)?;
        }
        Ok(())
    }
}
