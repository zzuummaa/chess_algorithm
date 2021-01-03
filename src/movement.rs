#![allow(dead_code)]

use crate::point::Point;
use std::mem::MaybeUninit;
use crate::board::ByteBoard;
use crate::figure_list::FigureList;
use crate::figure::{Color, Rank};
use crate::figure::Rank::{OUT, PAWN};
use std::slice::Iter;

#[derive(Debug)]
pub struct Move {
    pub from: Point,
    pub to: Point
}

pub trait Generator {
    fn fill(&self, move_list: &mut MoveList);
}

#[derive(Debug)]
pub struct MoveList {
    len: usize,
    buffer: [Move; 150]
}

impl MoveList {
    pub fn new<T: Generator>(generator: &T) -> Self {
        let mut move_list = MoveList::default();
        generator.fill(&mut move_list);
        return move_list;
    }

    pub fn push(&mut self, m: Move) {
        self.buffer[self.len] = m;
        self.len += 1
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        self.len = 0
    }

    pub fn iter(&self) -> Iter<'_, Move> {
        self.buffer[..self.len].iter()
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
}

static KING_MOVES_X: [i8; 8] = [ 0, 1, 1, 0, -1, -1, -1, 1 ];
static KING_MOVES_Y: [i8; 8] = [ 1, 0, 1, -1, 0, -1, 1, -1 ];

static KNIGHT_MOVES_X: [i8; 8] = [ 1, 2, -1, 2, 1, -2, -1, -2];
static KNIGHT_MOVES_Y: [i8; 8] = [ 2, 1, 2, -1, -2, 1, -2, -1];

static ROOK_DIRECTIONS_X: [i8; 4] = [ 0, 1, -1, 0 ];
static ROOK_DIRECTIONS_Y: [i8; 4] = [ 1, 0, 0, -1 ];

static BISHOP_DIRECTIONS_X: [i8; 4] = [ 1, -1, 1, -1 ];
static BISHOP_DIRECTIONS_Y: [i8; 4] = [ 1, 1, -1, -1 ];

impl<'a> Generator for MoveGenerator<'a> {
    fn fill(&self, move_list: &mut MoveList) {
        println!("{}\n", self.board);
        move_list.clear();
        self.figures.iter().for_each(|p| {
            self.fill_for_figure(p, move_list);
        });
    }
}

impl<'a> MoveGenerator<'a> {
    pub fn new(board: &'a ByteBoard, figures: &'a FigureList) -> Self {
        MoveGenerator { board, figures }
    }

    pub fn fill_for_figure(&self, p: Point, move_list: &mut MoveList) {
        let f =  self.board.point(p);
        match f.rank() {
            Rank::KING => {
                self.generate_moves(p, &KING_MOVES_X, &KING_MOVES_Y, move_list);
            }
            Rank::QUEEN => {
                self.generate_directions_moves(p, &ROOK_DIRECTIONS_X, &ROOK_DIRECTIONS_Y, move_list);
                self.generate_directions_moves(p, &BISHOP_DIRECTIONS_X, &BISHOP_DIRECTIONS_Y, move_list);
            }
            Rank::ROOK => {
                self.generate_directions_moves(p, &ROOK_DIRECTIONS_X, &ROOK_DIRECTIONS_Y, move_list);
            }
            Rank::BISHOP => {
                self.generate_directions_moves(p, &BISHOP_DIRECTIONS_X, &BISHOP_DIRECTIONS_Y, move_list);
            }
            Rank::KNIGHT => {
                self.generate_moves(p, &KNIGHT_MOVES_X, &KNIGHT_MOVES_Y, move_list);
            }
            Rank::PAWN => {
                let eat_color;
                let mult = match f.color() {
                    Color::NONE => unreachable!(),
                    Color::WHITE => {
                        eat_color = Color::BLACK;
                        1i8
                    },
                    Color::BLACK => {
                        eat_color = Color::WHITE;
                        -1i8
                    }
                };

                let eat_p = p + Point::new(1, mult);
                if self.board.point(eat_p).color() == eat_color { move_list.push(Move { from: p, to: eat_p }) }

                let eat_p = p + Point::new(-1, mult);
                if self.board.point(eat_p).color() == eat_color { move_list.push(Move { from: p, to: eat_p }) }

                self.move_if_not_out(p, 0, mult).iter().for_each(|to_p| {
                    move_list.push(Move { from: p, to: *to_p });
                });

                if p.y() == 1i8 && mult == 1 || p.y() == 6i8 && mult == -1 {
                    move_list.push(Move { from: p, to: p + Point::new(0, mult * 2) })
                }
            }
            Rank::NONE => unreachable!(),
            Rank::OUT => unreachable!()
        }

        // if move_list.iter().last().is_some() {
        //     println!("{} - {:?}", f, move_list.iter().last().unwrap());
        // }
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
        let f_color = self.board.point(p).color();
        movies_x.iter()
            .zip(movies_y.iter())
            .filter_map(|dp| self.move_if_not_out(p, *dp.0, *dp.1))
            .filter(|to_p| f_color != self.board.point(*to_p).color())
            .for_each(|to_p| {
                move_list.push(Move { from: p, to: to_p});
            });
    }

    fn generate_directions_moves(&self, p: Point, directions_x: &[i8; 4], directions_y: &[i8; 4], move_list: &mut MoveList) {
        let f_color = self.board.point(p).color();
        directions_x.iter()
            .zip(directions_y.iter())
            .for_each(|d| {
                let mut to_p = p;
                loop {
                    match self.move_if_not_out(to_p, *d.0, *d.1) {
                        None => break,
                        Some(new_to_p) => {
                            let to_color = self.board.point(new_to_p).color();
                            if to_color == f_color { break; }
                            move_list.push(Move { from: p, to: new_to_p });
                            if to_color != f_color { break; }
                            to_p = new_to_p
                        }
                    }
                }
            });
    }
}
