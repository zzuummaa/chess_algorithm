#![allow(dead_code)]

use crate::point::Point;
use std::mem::MaybeUninit;
use crate::board::ByteBoard;
use crate::figure_list::FigureList;
use crate::figure::{Color, Rank};
use crate::figure::Rank::OUT;

#[derive(Debug)]
pub struct Move {
    from: Point,
    to: Point
}

#[derive(Debug)]
pub struct MoveList {
    len: usize,
    buffer: [Move; 150]
}

impl MoveList {
    pub fn push(&mut self, m: Move) {
        self.buffer[self.len] = m;
        self.len += 1
    }

    pub fn clear(&mut self) {
        self.len = 0
    }
}

impl Default for MoveList {
    fn default() -> Self {
        MoveList {
            len: 0,
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

static KING_MOVES_X: [i8; 8] = [ 0, 1, 1, 0, -1, -1, -1, 1 ];
static KING_MOVES_Y: [i8; 8] = [ 1, 0, 1, -1, 0, -1, 1, -1 ];

static KNIGHT_MOVES_X: [i8; 8] = [ 1, 2, -1, 2, 1, -2, -1, -2];
static KNIGHT_MOVES_Y: [i8; 8] = [ 2, 1, 2, -1, -2, 1, -2, -1];

impl<'a> MoveGenerator<'a> {
    pub fn generate(&self, move_list: &mut MoveList) {
        move_list.clear();
        self.figures.iter().for_each(|p| {
            self.generate_for_figure(p, move_list);
        });
    }

    pub fn generate_for_figure(&self, p: Point, move_list: &mut MoveList) {
        let f =  self.board.point(p);
        match f.rank() {
            Rank::KING => {
                self.generate_moves(p, &KING_MOVES_X, &KING_MOVES_Y, move_list);
            }
            Rank::QUEEN => {}
            Rank::ROOK => {}
            Rank::BISHOP => {}
            Rank::KNIGHT => {
                self.generate_moves(p, &KNIGHT_MOVES_X, &KNIGHT_MOVES_X, move_list);
            }
            Rank::PAWN => {}
            Rank::NONE => unreachable!(),
            Rank::OUT => unreachable!()
        }
    }

    pub fn move_if_not_out(&self, p: Point, dx: i8, dy: i8) -> Option<Point> {
        let p_move = p + Point::new(dx, dy);
        if self.board.point(p_move).rank() != OUT {
            Some(p_move)
        } else {
            None
        }
    }

    fn generate_moves(&self, p: Point, movies_x: &[i8; 8], movies_y: &[i8; 8], move_list: &mut MoveList) {
        movies_x.iter()
            .zip(movies_y.iter())
            .filter_map(|dp| self.move_if_not_out(p, *dp.0, *dp.1))
            .for_each(|to_p| move_list.push(Move { from: p, to: to_p}) );
    }
}