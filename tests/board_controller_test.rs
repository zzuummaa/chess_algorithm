extern crate chess_algorithm;

use chess_algorithm::board::ByteBoard;
use chess_algorithm::board_controller::BoardDataHolder;
use chess_algorithm::figure::Color::WHITE;
use chess_algorithm::figure::Figure;
use chess_algorithm::figure::Rank::PAWN;
use chess_algorithm::movement::{Move, MoveType};
use chess_algorithm::point::Point;

#[test]
fn transform_test() {
    let mut board = ByteBoard::empty();
    *board.cell_mut(1, 6) = Figure::new(PAWN, WHITE, false);

    let movement = Move {
        from: Point::new(1, 6),
        to: Point::new(1, 7),
        m_type: MoveType::TRANSFORM
    };

    let mut holder =  BoardDataHolder::new(&board);

    let info = holder.controller(WHITE).make_move(&movement);

    assert_eq!(holder.white_list.iter().last().unwrap(), Point::new(1, 7));

    holder.controller(WHITE).unmake_move(info);

    assert_eq!(board, holder.board);
    assert_eq!(holder.white_list.iter().last().unwrap(), Point::new(1, 6));
}