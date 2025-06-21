#![feature(test)]
extern crate chess_algorithm;
extern crate test;

use test::Bencher;

use chess_algorithm::board::ByteBoard;
use chess_algorithm::board_controller::*;
use chess_algorithm::figure::Color::*;
use chess_algorithm::score::{AlphaBetaSearch, MinMaxSimpleSearch, MoveSearch};

#[bench]
fn bench_alpha_betta(b: &mut Bencher) {
    let mut board_data_holder = BoardDataHolder::new(&ByteBoard::default());
    b.iter(|| {
        let mut controller = board_data_holder.controller(WHITE);
        AlphaBetaSearch::default().find_best_move(&mut controller, 4);
    });
}

#[bench]
fn bench_min_max_simple(b: &mut Bencher) {
    let mut board_data_holder = BoardDataHolder::new(&ByteBoard::default());
    b.iter(|| {
        let mut controller = board_data_holder.controller(WHITE);
        MinMaxSimpleSearch::default().find_best_move(&mut controller, 4);
    });
}