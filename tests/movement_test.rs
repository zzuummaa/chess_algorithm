#![feature(is_sorted)]

extern crate chess_algorithm;
use chess_algorithm::movement::*;
use chess_algorithm::board::ByteBoard;
use chess_algorithm::figure::{Figure, Color};
use chess_algorithm::figure::Rank::{KING, PAWN, QUEEN, ROOK, NONE};
use chess_algorithm::figure::Color::{WHITE, BLACK};
use chess_algorithm::figure_list::{FigurePointerList};
use std::collections::HashSet;
use chess_algorithm::point::Point;
use chess_algorithm::board_controller::BoardDataHolder;
use chess_algorithm::score::simple_positional_fn;

struct DataHolder {
    board: ByteBoard,
    white_list: FigurePointerList,
    black_list: FigurePointerList,
    move_list: MoveList
}

impl DataHolder {
    fn new() -> DataHolder {
        DataHolder {
            board: ByteBoard::empty(),
            white_list: FigurePointerList::default(),
            black_list: FigurePointerList::default(),
            move_list: MoveList::default()
        }
    }

    fn generate_figure_movies(&mut self, x: i8, y: i8) -> &MoveList {
        MoveGenerator::new(&self.board, &self.white_list).fill_for_figure(Point::new(x, y), &mut self.move_list);
        &mut self.move_list
    }

    fn generate_white_movies(&mut self) -> &MoveList {
        self.white_list.fill(&self.board, WHITE);
        MoveGenerator::new(&self.board, &self.white_list).fill(&mut self.move_list);
        &self.move_list
    }

    fn generate_black_movies(&mut self) -> &MoveList {
        self.black_list.fill(&self.board, BLACK);
        MoveGenerator::new(&self.board, &self.black_list).fill(&mut self.move_list);
        &self.move_list
    }

    fn sort_moves(&mut self) {
        self.move_list.sort_by(&self.board, simple_positional_fn);
    }
}

#[test]
fn test_pointers() {
    let mut data_holder = BoardDataHolder::new(&ByteBoard::default());
    let controller = data_holder.controller(WHITE);
    let move_list = MoveList::default();
    controller.is_king_alive();
    move_list.iter().count();
}

#[test]
fn test_generate_king_movies_without_outs() {
    let mut expected_movies = HashSet::new();

    expected_movies.insert(Point::new(0, 0));
    expected_movies.insert(Point::new(1, 0));
    expected_movies.insert(Point::new(2, 0));
    expected_movies.insert(Point::new(2, 1));
    expected_movies.insert(Point::new(2, 2));
    expected_movies.insert(Point::new(1, 2));
    expected_movies.insert(Point::new(0, 2));
    expected_movies.insert(Point::new(0, 1));

    let mut data_holder = DataHolder::new();
    *data_holder.board.cell_mut(1, 1) = Figure::new(KING, WHITE, false);
    let movies: HashSet<Point> = data_holder.generate_white_movies().iter().map(|m| m.to).collect();

    assert_eq!(movies, expected_movies);
}

#[test]
fn test_generate_king_movies_from_conner() {
    let mut expected_movies = HashSet::new();

    expected_movies.insert(Point::new(1, 0));
    expected_movies.insert(Point::new(1, 1));
    expected_movies.insert(Point::new(0, 1));

    let mut data_holder = DataHolder::new();
    *data_holder.board.cell_mut(0, 0) = Figure::new(KING, WHITE, false);
    let movies: HashSet<Point> = data_holder.generate_white_movies().iter().map(|m| m.to).collect();

    assert_eq!(movies, expected_movies);
}

#[test]
fn test_generate_king_movies_with_friend_figure() {
    let mut expected_movies = HashSet::new();

    expected_movies.insert(Point::new(1, 0));
    expected_movies.insert(Point::new(0, 1));

    let mut data_holder = DataHolder::new();
    *data_holder.board.cell_mut(0, 0) = Figure::new(KING, WHITE, false);
    *data_holder.board.cell_mut(1, 1) = Figure::new(PAWN, WHITE, false);
    let movies: HashSet<Point> = data_holder.generate_figure_movies(0, 0).iter().map(|m| m.to).collect();

    assert_eq!(movies, expected_movies);
}

#[test]
fn test_rook_movies() {
    let mut expected_movies = HashSet::new();

    for y in 0..8 {
        if y == 1 { continue; }
        expected_movies.insert(Point::new(1, y));
    }
    for x in 0..8 {
        if x == 1 { continue; }
        expected_movies.insert(Point::new(x, 1));
    }

    let mut data_holder = DataHolder::new();
    *data_holder.board.cell_mut(1, 1) = Figure::new(ROOK, WHITE, false);
    let movies: HashSet<Point> = data_holder.generate_figure_movies(1, 1).iter().map(|m| m.to).collect();

    assert_eq!(movies, expected_movies);
}

#[test]
fn test_rook_eat() {
    let mut data_holder = DataHolder::new();
    *data_holder.board.cell_mut(1, 1) = Figure::new(ROOK, WHITE, false);
    *data_holder.board.cell_mut(1, 6) = Figure::new(PAWN, BLACK, false);
    let movies: HashSet<Move> = data_holder.generate_figure_movies(1, 1).iter().map(|m| *m).collect();

    assert!(movies.contains(&Move { from: Point::new(1, 1), to: Point::new(1, 6), m_type: MoveType::SIMPLE }));
}

#[test]
fn test_generate_queen_take() {
    let mut data_holder = DataHolder::new();
    *data_holder.board.cell_mut(1, 1) = Figure::new(QUEEN, WHITE, false);
    *data_holder.board.cell_mut(1, 6) = Figure::new(PAWN, BLACK, false);
    let movies: HashSet<Move> = data_holder.generate_figure_movies(1, 1).iter().map(|m| *m).collect();

    assert!(movies.contains(&Move { from: Point::new(1, 1), to: Point::new(1, 6), m_type: MoveType::SIMPLE }));
}

#[test]
fn test_pawn_first_moves() {
    let mut expected_movies = HashSet::new();

    expected_movies.insert(Point::new(1, 2));
    expected_movies.insert(Point::new(1, 3));

    let mut data_holder = DataHolder::new();
    *data_holder.board.cell_mut(1, 1) = Figure::new(PAWN, WHITE, false);
    let movies: HashSet<Point> = data_holder.generate_figure_movies(1, 1).iter().map(|m| m.to).collect();

    assert_eq!(movies, expected_movies);
}

#[test]
fn test_pawn_first_moves_with_let() {
    let mut expected_movies = HashSet::new();

    let mut data_holder = DataHolder::new();
    *data_holder.board.cell_mut(1, 1) = Figure::new(PAWN, WHITE, false);
    *data_holder.board.cell_mut(1, 2) = Figure::new(PAWN, BLACK, false);
    assert_eq!(data_holder.generate_figure_movies(1, 1).iter().map(|m| m.to).count(), 0);

    expected_movies.insert(Point::new(1, 2));

    *data_holder.board.cell_mut(1, 2) = Figure::new(NONE, Color::NONE, false);
    *data_holder.board.cell_mut(1, 3) = Figure::new(PAWN, BLACK, false);

    let movies: HashSet<Point> = data_holder.generate_figure_movies(1, 1).iter().map(|m| m.to).collect();
    assert_eq!(movies, expected_movies);
}

#[test]
fn test_is_movement_list_descending_sort() {
    let mut data_holder = DataHolder::new();
    *data_holder.board.cell_mut(1, 1) = Figure::new(QUEEN, WHITE, false);
    *data_holder.board.cell_mut(1, 6) = Figure::new(PAWN, BLACK, false);

    data_holder.generate_figure_movies(1, 1);
    data_holder.sort_moves();

    let movies_scores: Vec<_> = data_holder.move_list.iter().map(|m| {
        let f = *data_holder.board.point(m.from);
        let d_score = simple_positional_fn(m.to, f) - simple_positional_fn(m.from, f) + data_holder.board.point(m.to).weight();
        (m, d_score)
    }).collect();

    movies_scores.iter().for_each(|(m, s)| {
        println!("{}, {} -> {}, d_score: {}", m, data_holder.board.point(m.from), data_holder.board.point(m.to), s);
    });

    assert!(movies_scores.iter().map(|(m, s)| s).is_sorted_by(|a, b| Some(b.cmp(a))));
}