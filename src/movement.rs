#![allow(dead_code)]

use crate::point::Point;
use std::mem::MaybeUninit;
use crate::board::ByteBoard;
use crate::figure_list::FigureList;
use crate::figure::{Color, Rank};

#[derive(Debug)]
pub struct Move {
    from: Point,
    to: Point
}

#[derive(Debug)]
pub struct MoveList {
    buffer: [Move; 150]
}

impl Default for MoveList {
    fn default() -> Self {
        MoveList {
            // TODO reduce overhead for initialization
            buffer: unsafe { MaybeUninit::uninit().assume_init() }
        }
    }
}

pub struct MoveGenerator<'a> {
    pub board: &'a ByteBoard,
    pub figures: &'a FigureList,
    pub color: Color
}

impl<'a> MoveGenerator<'a> {
    pub fn generate(&mut self) -> MoveList {
        let move_list = MoveList::default();
        self.figures.iter().for_each(|p| {
            let f =  self.board.point(p);
            match f.rank() {
                Rank::KING => {}
                Rank::QUEEN => {}
                Rank::ROOK => {}
                Rank::BISHOP => {}
                Rank::KNIGHT => {}
                Rank::PAWN => {}
                Rank::NONE => unreachable!(),
                Rank::OUT => unreachable!()
            }
        });
        move_list
    }
}