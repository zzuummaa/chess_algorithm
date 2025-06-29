use chess_algorithm::board::ByteBoard;
use chess_algorithm::board_controller::BoardDataHolder;
use chess_algorithm::figure::Color::WHITE;
use chess_algorithm::figure::Figure;
use chess_algorithm::figure::Rank::PAWN;
use chess_algorithm::score::min_max_simple;

#[test]
fn test_simple_min_max() {
    let mut board = ByteBoard::empty();
    *board.cell_mut(1, 6) = Figure::new(PAWN, WHITE, false);

    let mut holder =  BoardDataHolder::new(&board);
    let mut controller = holder.controller(WHITE);
    min_max_simple(&mut controller, 4);
}