use chess_algorithm::score_estimator::ScoreEstimator;
use chess_algorithm::figure::Color::WHITE;
use chess_algorithm::board::ByteBoard;
use chess_algorithm::figure::Figure;
use chess_algorithm::figure::Rank::PAWN;

#[test]
fn test_simple_min_max() {
    let mut board = ByteBoard::empty();
    *board.cell_mut(1, 1) = Figure::new(PAWN, WHITE);
    let mut score_estimator = ScoreEstimator::new(&board);
    score_estimator.min_max_simple(3, WHITE);
}