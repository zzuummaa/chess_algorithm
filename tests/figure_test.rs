extern crate chess_algorithm;
use chess_algorithm::figure::*;

#[test]
fn test_build_figure() {
    let figure = build_figure(Rank::NONE, Color::WHITE);
    assert_eq!(figure.rank(), Rank::NONE);
    assert_eq!(figure.color(), Color::WHITE);

    let figure = build_figure(Rank::KING, Color::WHITE);
    assert_eq!(figure.rank(), Rank::KING);
    assert_eq!(figure.color(), Color::WHITE);

    let figure = build_figure(Rank::KNIGHT, Color::BLACK);
    assert_eq!(figure.rank(), Rank::KNIGHT);
    assert_eq!(figure.color(), Color::BLACK);

    let figure = build_figure(Rank::PAWN, Color::BLACK);
    assert_eq!(figure.rank(), Rank::PAWN);
    assert_eq!(figure.color(), Color::BLACK);
}

#[test]
fn test_weight_figure() {
    let figure = build_figure(Rank::NONE, Color::WHITE);
    assert_eq!(figure.weight(), 0);

    let figure = build_figure(Rank::PAWN, Color::WHITE);
    assert_eq!(figure.weight(), W_PAWN);

    let figure = build_figure(Rank::QUEEN, Color::WHITE);
    assert_eq!(figure.weight(), W_QUEEN);

    let figure = build_figure(Rank::KING, Color::WHITE);
    assert_eq!(figure.weight(), W_KING);
}