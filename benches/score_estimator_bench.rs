#![feature(test)]
extern crate test;
extern crate chess_algorithm;

use test::Bencher;
use chess_algorithm::score_estimator::*;
use chess_algorithm::board::ByteBoard;
use chess_algorithm::figure::Color::{WHITE, BLACK};
use chess_algorithm::figure_list::FigureList;

#[bench]
fn bench_node_iterations(b: &mut Bencher) {
    let board = ByteBoard::default();
    let mut white_list = FigureList::new(&board, WHITE);
    let mut black_list = FigureList::new(&board, BLACK);
    b.iter(|| {
        let mut score_estimator = ScoreEstimator::new(&board);
        score_estimator.min_max_simple(6, &mut white_list, &mut black_list, WHITE);
    });
}