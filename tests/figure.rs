extern crate chess_algorithm;
use chess_algorithm::figure::*;

#[test]
fn test_build_figure() {
    let figure = build_figure(FigureRank::KING);
    assert_eq!(figure.rank(), FigureRank::KING);
}