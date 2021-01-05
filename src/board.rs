#![allow(dead_code)]

use crate::point::*;
use crate::movement::Move;
use crate::figure::*;
use std::fmt;
use std::fmt::{Display, Formatter, Debug};
use crate::figure::Rank::NONE;

#[derive(Debug, Copy, Clone)]
pub struct ByteBoard {
    /// First index is letter,
    /// Second index is number
    /// Example: A2 -> cells[0][1]
    cells: [[Figure; 16]; 16],
}

impl ByteBoard {
    pub fn empty() -> Self {
        let mut board = ByteBoard {
            cells: [[Figure::new(Rank::OUT, Color::NONE); 16]; 16],
        };

        for i in 0..8 {
            for j in 0..8 {
                *board.cell_mut(i, j) = Figure::new(Rank::NONE, Color::NONE);
            }
        }

        board
    }

    pub fn make_move(&mut self, movement: &Move) -> Figure {
        let old_to = *self.point(movement.to);
        *self.point_mut(movement.to) = *self.point(movement.from);
        *self.point_mut(movement.from) = Figure::new(NONE, Color::NONE);
        return old_to;
    }

    pub fn unmake_move(&mut self, movement: &Move, figure: Figure) {
        *self.point_mut(movement.from) = *self.point(movement.to);
        *self.point_mut(movement.to) = figure;
    }

    pub fn cell_mut(&mut self, literal: isize, number: isize) -> &mut Figure {
        debug_assert!(literal < 12 && literal >= -4);
        debug_assert!(number < 12 && number >= -4);
        &mut self.cells[(literal + 4) as usize][(number + 4) as usize]
    }

    pub fn cell(&self, literal: isize, number: isize) -> &Figure {
        debug_assert!(literal < 12 && literal >= -4);
        debug_assert!(number < 12 && number >= -4);
        &self.cells[(literal + 4) as usize][(number + 4) as usize]
    }

    pub fn point(&self, point: Point) -> &Figure {
        self.cell(point.x() as isize, point.y() as isize)
    }

    pub fn point_mut(&mut self, point: Point) -> &mut Figure {
        self.cell_mut(point.x() as isize, point.y() as isize)
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

impl Default for ByteBoard {
    fn default() -> Self {
        let mut board = ByteBoard {
            cells: [[Figure::new(Rank::OUT, Color::NONE); 16]; 16],
        };

        for i in 0..8 {
            for j in 2..6 {
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
}

impl Display for ByteBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (s, e) = (0isize, 8isize);
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
