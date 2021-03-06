extern crate chess_algorithm;
use chess_algorithm::figure::*;
use std::mem;

#[test]
fn test_size_figure() {
    assert_eq!(mem::size_of::<Figure>(), 1);
}

#[test]
fn test_build_figure() {
    let figure = Figure::new(Rank::NONE, Color::WHITE, false);
    assert_eq!(figure.rank(), Rank::NONE);
    assert_eq!(figure.color(), Color::WHITE);

    let figure = Figure::new(Rank::KING, Color::WHITE, false);
    assert_eq!(figure.rank(), Rank::KING);
    assert_eq!(figure.color(), Color::WHITE);

    let figure = Figure::new(Rank::KNIGHT, Color::BLACK, false);
    assert_eq!(figure.rank(), Rank::KNIGHT);
    assert_eq!(figure.color(), Color::BLACK);

    let figure = Figure::new(Rank::PAWN, Color::BLACK, false);
    assert_eq!(figure.rank(), Rank::PAWN);
    assert_eq!(figure.color(), Color::BLACK);
}

#[test]
fn test_weight_figure() {
    let figure = Figure::new(Rank::NONE, Color::WHITE, false);
    assert_eq!(figure.weight(), 0);

    let figure = Figure::new(Rank::PAWN, Color::WHITE, false);
    assert_eq!(figure.weight(), W_PAWN);

    let figure = Figure::new(Rank::QUEEN, Color::WHITE, false);
    assert_eq!(figure.weight(), W_QUEEN);

    let figure = Figure::new(Rank::KING, Color::WHITE, false);
    assert_eq!(figure.weight(), W_KING);
}

#[test]
fn test_flag_figure() {
    let figure = Figure::new(Rank::NONE, Color::WHITE, false);
    assert_eq!(figure.is_flag_set(), false);

    let figure = Figure::new(Rank::NONE, Color::WHITE, true);
    assert_eq!(figure.is_flag_set(), true);
}