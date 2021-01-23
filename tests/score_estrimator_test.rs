use chess_algorithm::board::ByteBoard;
use chess_algorithm::figure::Color::WHITE;
use chess_algorithm::board_controller::{BoardController, BoardDataHolder};

#[test]
fn test_simple_min_max() {
    let board = ByteBoard::default();
    let mut board_data_holder =  BoardDataHolder::new(&board);
    board_data_holder
        .controller(WHITE)
        .min_max_simple(6);
}