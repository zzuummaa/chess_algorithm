extern crate chess_algorithm;
use chess_algorithm::movement::*;
use chess_algorithm::board::ByteBoard;
use chess_algorithm::figure::Figure;
use chess_algorithm::figure::Rank::KING;
use chess_algorithm::figure::Color::WHITE;
use chess_algorithm::figure_list::FigureList;

#[test]
fn test_move_list_default() {
    println!("{:?}", MoveList::default())
}

#[test]
fn test_generate_king_movies() {
    let mut board = ByteBoard::empty();
    *board.cell_mut(1, 1) = Figure::new(KING, WHITE);

    let mut figure_list = FigureList::new();
    figure_list.fill(&board, WHITE);

    let mut move_list = MoveList::default();

    let move_generator = MoveGenerator::new(&board, &figure_list, WHITE);
    move_generator.generate(&mut move_list);

    assert_eq!(move_list.len(), 8);
}