use chess_algorithm::board::ByteBoard;
use chess_algorithm::figure::{Color, Figure, W_INFINITY};
use chess_algorithm::figure::Color::{BLACK, WHITE};
use chess_algorithm::figure::Rank::{KNIGHT, PAWN, KING, BISHOP, ROOK};
use chess_algorithm::figure_list::FigurePointerList;
use chess_algorithm::movement::{Move, MoveGenerator, MoveList};

fn evaluate_score(board: &ByteBoard, friend_list: &FigurePointerList, enemy_list: &FigurePointerList) -> i32 {
    let friend_score: i32 = friend_list.iter()
        .map(|p| board.point(p).weight())
        .sum();
    // println!("friend_score: {}", friend_score);

    let enemy_score: i32 = enemy_list.iter()
        .map(|p| board.point(p).weight())
        .sum();
    // println!("enemy_score: {}", enemy_score);

    friend_score - enemy_score
}

fn recursion(board: &mut ByteBoard, depth: i32, friend_list: &mut FigurePointerList, enemy_list: &mut FigurePointerList, friend_color: Color) -> (Option<Move>, i32) {
    if depth <= 0 {
        return (None, evaluate_score(board, friend_list, enemy_list))
    }

    let enemy_color = friend_color.invert();
    let mut best_move: Option<Move> = None;
    let mut best_score = - W_INFINITY;

    let move_list = MoveList::new(&MoveGenerator::new(board, friend_list));

    for movement in move_list.iter() {
        let to_figure = board.make_move(movement);
        let fl_restore_info = friend_list.make_move(movement);

        let mut el_restore_info = Default::default();
        if to_figure.color() == enemy_color {
            el_restore_info =
                enemy_list.remove(movement.to);
        }

        let cur_score = - recursion(board, depth-1, enemy_list, friend_list, enemy_color).1;
        if cur_score > best_score {
            best_move = Some(*movement);
            best_score = cur_score;
        }

        board.unmake_move(movement, to_figure);
        friend_list.unmake_move(movement, fl_restore_info);
        if to_figure.color() == enemy_color {
            enemy_list.restore(el_restore_info);
        }
    }

    (best_move, best_score)
}

#[test]
fn test_linked_list_in_recursion() {
    let mut board = ByteBoard::empty();
    *board.cell_mut(1, 1) = Figure::new(KNIGHT, WHITE);
    *board.cell_mut(1, 2) = Figure::new(BISHOP, WHITE);
    *board.cell_mut(6, 6) = Figure::new(KING, BLACK);
    *board.cell_mut(6, 7) = Figure::new(ROOK, BLACK);
    *board.cell_mut(4, 4) = Figure::new(PAWN, BLACK);

    let mut white_list = FigurePointerList::new(&board, WHITE);
    let mut black_list = FigurePointerList::new(&board, BLACK);

    recursion(&mut board, 30, &mut white_list, &mut black_list, WHITE);
}