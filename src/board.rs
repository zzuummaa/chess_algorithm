#![allow(dead_code)]

use crate::point::*;
use crate::figure::*;
use crate::figure_list::*;
use std::fmt;
use std::fmt::{Display, Formatter, Debug};

#[derive(Debug)]
pub struct ByteBoard {
    /// First index is letter,
    /// Second index is number
    /// Example: A2 -> cells[0][1]
    cells: [[Figure; 8]; 8],

    pub white_list: FigureList,
    pub black_list: FigureList,
}

impl ByteBoard {
    pub fn new() -> Self {
        let mut board = ByteBoard {
            cells: [[Figure::new(Rank::NONE, Color::NONE); 8]; 8],
            white_list: FigureList::new(),
            black_list: FigureList::new()
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

    pub fn cell(&self, literal: usize, number: usize) -> Figure {
        self.cells[literal][number]
    }

    pub fn point(&self, point: Point) -> Figure {
        self.cells[point.x as usize][point.y as usize]
    }

    pub fn cell_iter(&self) -> impl Iterator<Item = (Point, &Figure)> {
        self.cells.iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(y, column)| (Point { x: x as u8, y: y as u8}, column))
            })
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
