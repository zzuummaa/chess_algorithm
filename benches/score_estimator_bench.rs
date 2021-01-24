#![feature(test)]
extern crate chess_algorithm;
extern crate test;

use test::Bencher;

use chess_algorithm::board::ByteBoard;
use chess_algorithm::figure::Color::*;
use chess_algorithm::board_controller::*;
use chess_algorithm::figure::W_INFINITY;

#[bench]
fn bench_node_iterations(b: &mut Bencher) {
    let mut board_data_holder = BoardDataHolder::new(&ByteBoard::default());
    b.iter(|| {
        board_data_holder.controller(WHITE).alpha_betta(5, -(W_INFINITY), (W_INFINITY));
    });
}