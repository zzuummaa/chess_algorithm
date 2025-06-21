//! `ByteBoard` represents a chess-like board with an extended 16x16 grid.
//! The board primarily operates within the central 8x8 area to simulate
//! a traditional chessboard, while the outer regions provide padding for
//! special calculations or validations.
//!
//! # Fields
//! - `cells`: A 16x16 grid of `Figure` objects, where the first index represents
//!   the column (letters A-P) and the second index represents the row (numbers 1-16).
//!   For example, cell `A2` corresponds to `cells[0][1]`.
//!
//! # Traits
//! - `PartialEq`: Compares two boards for equality by checking if all their cells are the same.
//! - `Copy` and `Clone`: Implements copy semantics to duplicate the board's state.
//! - `Debug`: Implements debugging output using the `derive` macro.
//! - `Default`: Provides an initial chess setup on the board for standard gameplay.
//!
//! # Methods
//! ## Constructors
//! - `empty() -> Self`:
//!   Creates a board with all cells initialized to an empty `Figure` with `Rank::NONE` and `Color::NONE`.
//!
//! - `default() -> Self`:
//!   Creates a board initialized with a standard chess configuration,
//!   including pawns and special pieces such as rooks, knights, bishops, queens, and kings.
//!
//! ## Mutator Methods
//! - `cell_mut(&mut self, literal: isize, number: isize) -> &mut Figure`:
//!   Returns a mutable reference to a specific cell on the board given its (literal, number) coordinates.
//!   The inputs are valid within the extended 16x16 bounds centered on the main 8x8 area.
//!   **Panics** in debug mode if the inputs are out of bounds (between -4 and 11 inclusive).
//!
//! - `point_mut(&mut self, point: Point) -> &mut Figure`:
//!   Returns a mutable reference to the cell specified by a `Point` object.
//!
//! - `swap(&mut self, p1: Point, p2: Point)`:
//!   Swaps the figures at two specified `Point` locations.
//!
//! ## Accessor Methods
//! - `cell(&self, literal: isize, number: isize) -> &Figure`:
//!   Returns a reference to the figure at a specific (literal, number) coordinate.
//!   Same boundary restrictions and behavior as `cell_mut()`.
//!
//! - `point(&self, point: Point) -> &Figure`:
//!   Returns a read-only reference to the cell at the `Point` object's coordinates.
//!
//! - `cell_iter(&self) -> impl Iterator<Item = (Point, &Figure)>`:
//!   Returns an iterator over the main 8x8 area of the board, yielding pairs of `Point` and `Figure` references.
//!
//! ## Display
//! - Implements the `Display` trait to format the board as a human-readable string, 
//!   with rows labeled 1-8 and columns labeled A-H for the main 8x8 area.
//!   The display output may look similar to a chessboard, simplifying visualization or debugging.
//!
//! ## Examples
//! ```
//! use chess_algorithm::board::ByteBoard;
//! use chess_algorithm::point::Point;
//! let mut board = ByteBoard::default();
//! println!("{}", board); // Prints the default board configuration.
//! let point1 = Point::new(3, 1); // D2
//! let point2 = Point::new(3, 2); // D3
//! board.swap(point1, point2);
//! println!("{}", board); // Prints board after swapping figures.
//! ```

#![allow(dead_code)]

use crate::point::*;
use crate::figure::*;
use std::fmt;
use std::fmt::{Display, Formatter, Debug};

#[derive(Debug, Copy, Clone)]
pub struct ByteBoard {
    /// First index is letter,
    /// Second index is number
    /// Example: A2 -> cells[4][5]
    cells: [[Figure; 16]; 16],
}

impl PartialEq for ByteBoard {
    fn eq(&self, other: &ByteBoard) -> bool {
        self.cells.iter()
            .flatten()
            .zip(other.cells.iter().flatten())
            .all(|(a, b)| a == b)
    }
}

impl ByteBoard {
    pub fn empty() -> Self {
        let mut board = ByteBoard {
            cells: [[Figure::new(Rank::OUT, Color::NONE, false); 16]; 16],
        };

        for i in 0..8 {
            for j in 0..8 {
                *board.cell_mut(i, j) = Figure::new(Rank::NONE, Color::NONE, false);
            }
        }

        board
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

    pub fn swap(&mut self, p1: Point, p2: Point) {
        let f = *self.point(p1);
        *self.point_mut(p1) = *self.point(p2);
        *self.point_mut(p2) = f;
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
            cells: [[Figure::new(Rank::OUT, Color::NONE, false); 16]; 16],
        };

        for i in 0..8 {
            for j in 2..6 {
                *board.cell_mut(i, j) = Figure::new(Rank::NONE, Color::NONE, false);
            }
        }

        for i in 0..8 {
            *board.cell_mut(i, 1) = Figure::new(Rank::PAWN, Color::WHITE, false);
            *board.cell_mut(i, 6) = Figure::new(Rank::PAWN, Color::BLACK, false);
        }

        *board.cell_mut(0, 0) = Figure::new(Rank::ROOK, Color::WHITE, false);
        *board.cell_mut(7, 0) = Figure::new(Rank::ROOK, Color::WHITE, false);
        *board.cell_mut(0, 7) = Figure::new(Rank::ROOK, Color::BLACK, false);
        *board.cell_mut(7, 7) = Figure::new(Rank::ROOK, Color::BLACK, false);

        *board.cell_mut(1, 0) = Figure::new(Rank::KNIGHT, Color::WHITE, false);
        *board.cell_mut(6, 0) = Figure::new(Rank::KNIGHT, Color::WHITE, false);
        *board.cell_mut(1, 7) = Figure::new(Rank::KNIGHT, Color::BLACK, false);
        *board.cell_mut(6, 7) = Figure::new(Rank::KNIGHT, Color::BLACK, false);

        *board.cell_mut(2, 0) = Figure::new(Rank::BISHOP, Color::WHITE, true);
        *board.cell_mut(5, 0) = Figure::new(Rank::BISHOP, Color::WHITE, true);
        *board.cell_mut(2, 7) = Figure::new(Rank::BISHOP, Color::BLACK, true);
        *board.cell_mut(5, 7) = Figure::new(Rank::BISHOP, Color::BLACK, true);

        *board.cell_mut(4, 0) = Figure::new(Rank::QUEEN, Color::WHITE, false);
        *board.cell_mut(4, 7) = Figure::new(Rank::QUEEN, Color::BLACK, false);

        *board.cell_mut(3, 0) = Figure::new(Rank::KING, Color::WHITE, true);
        *board.cell_mut(3, 7) = Figure::new(Rank::KING, Color::BLACK, true);

        board
    }
}

impl Display for ByteBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (s, e) = (0isize, 8isize);
        for n in (s..e).rev() {
            write!(f, "{} ", n + 1)?;
            for l in (s..e).rev() {
                write!(f, "{} ", self.cell(l, n))?;
            }
            write!(f, "\n")?;
        }
        write!(f, "  ")?;
        for l in s..e {
            write!(f, "{}  ", (l as u8 + 65) as char)?;
        }
        Ok(())
    }
}
