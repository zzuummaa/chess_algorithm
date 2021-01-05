use chess_algorithm::board::ByteBoard;
use chess_algorithm::figure::Color::{BLACK, WHITE};
use chess_algorithm::figure::Figure;
use chess_algorithm::figure::Rank::{KNIGHT, PAWN};
use chess_algorithm::figure_list::FigurePointerList;
use chess_algorithm::score_estimator::ScoreEstimator;

#[test]
fn test_simple_min_max() {
    let board = ByteBoard::default();
    let mut white_list = FigurePointerList::new(&board, WHITE);
    let mut black_list = FigurePointerList::new(&board, BLACK);
    let mut score_estimator = ScoreEstimator::new(&board);
    score_estimator.min_max_simple(5, &mut white_list, &mut black_list, WHITE);
}