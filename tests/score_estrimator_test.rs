use chess_algorithm::score_estimator::ScoreEstimator;
use chess_algorithm::figure::Color::{WHITE, BLACK};
use chess_algorithm::board::ByteBoard;
use chess_algorithm::figure_list::FigureList;
use chess_algorithm::figure::Figure;
use chess_algorithm::figure::Rank::{QUEEN, KNIGHT};

#[test]
fn test_simple_min_max() {
    let mut board = ByteBoard::empty();
    *board.cell_mut(1, 1) = Figure::new(QUEEN, WHITE);
    *board.cell_mut(1, 6) = Figure::new(KNIGHT, BLACK);
    let mut white_list = FigureList::new(&board, WHITE);
    let mut black_list = FigureList::new(&board, BLACK);
    let mut score_estimator = ScoreEstimator::new(&board);
    score_estimator.min_max_simple(6, &mut white_list, &mut black_list, WHITE);
}