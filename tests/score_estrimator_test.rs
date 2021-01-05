use chess_algorithm::board::ByteBoard;
use chess_algorithm::figure::Color::WHITE;
use chess_algorithm::score_estimator::BoardDataHolder;

#[test]
fn test_simple_min_max() {
    let mut board_data_holder =  BoardDataHolder::new(&ByteBoard::default());
    board_data_holder
        .controller(WHITE)
        .min_max_simple(6);
}