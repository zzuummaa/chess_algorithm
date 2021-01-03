use chess_algorithm::score_estimator::ScoreEstimator;
use chess_algorithm::figure::Color::WHITE;
use chess_algorithm::board::ByteBoard;

#[test]
fn test_simple_min_max() {
    let mut board = ByteBoard::default();
    let mut score_estimator = ScoreEstimator::new(&board);
    score_estimator.min_max_simple(3, WHITE);
}