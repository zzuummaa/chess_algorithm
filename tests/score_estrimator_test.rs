use chess_algorithm::score_estimator::ScoreEstimator;
use chess_algorithm::figure::Color::{WHITE, BLACK};
use chess_algorithm::board::ByteBoard;
use chess_algorithm::figure_list::FigureList;

#[test]
fn test_simple_min_max() {
    let board = ByteBoard::default();
    let mut white_list = FigureList::new(&board, WHITE);
    let mut black_list = FigureList::new(&board, BLACK);
    let mut score_estimator = ScoreEstimator::new(&board);
    score_estimator.min_max_simple(5, &mut white_list, &mut black_list, WHITE);
}