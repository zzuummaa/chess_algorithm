use chess_algorithm::board::ByteBoard;
use chess_algorithm::figure::Color::WHITE;
use chess_algorithm::board_controller::BoardDataHolder;
use chess_algorithm::figure::Figure;
use chess_algorithm::figure::Rank::PAWN;

#[test]
fn test_simple_min_max() {
    let mut board = ByteBoard::empty();
    *board.cell_mut(1, 6) = Figure::new(PAWN, WHITE, false);

    let mut board_data_holder =  BoardDataHolder::new(&board);
    board_data_holder
        .controller(WHITE)
        .min_max_simple(5);
}