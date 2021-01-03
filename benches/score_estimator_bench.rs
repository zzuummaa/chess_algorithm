#![feature(test)]
extern crate test;
extern crate chess_algorithm;

use test::Bencher;
use chess_algorithm::score_estimator::*;
use chess_algorithm::board::ByteBoard;
use chess_algorithm::figure::Color::WHITE;

#[bench]
fn bench_node_iterations(b: &mut Bencher) {
    let board = ByteBoard::default();
    b.iter(|| {
        let mut score_estimator = ScoreEstimator::new(&board);
        score_estimator.min_max_simple(5, WHITE);
    });
}