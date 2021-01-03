extern crate chess_algorithm;
use chess_algorithm::movement::*;
use chess_algorithm::board::ByteBoard;
use chess_algorithm::figure::Figure;
use chess_algorithm::figure::Rank::{KING, PAWN};
use chess_algorithm::figure::Color::{WHITE, BLACK};
use chess_algorithm::figure_list::FigureList;
use std::collections::HashSet;
use chess_algorithm::point::Point;

struct DataHolder {
    board: ByteBoard,
    white_list: FigureList,
    black_list: FigureList,
    move_list: MoveList
}

impl DataHolder {
    fn new() -> DataHolder {
        DataHolder {
            board: ByteBoard::empty(),
            white_list: FigureList::default(),
            black_list: FigureList::default(),
            move_list: Default::default()
        }
    }

    fn generate_figure_movies(&mut self, x: i8, y: i8) -> &MoveList {
        MoveGenerator::new(&self.board, &self.white_list).fill_for_figure(Point::new(x, y), &mut self.move_list);
        &self.move_list
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
    *data_holder.board.cell_mut(1, 1) = Figure::new(KING, WHITE);
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
    *data_holder.board.cell_mut(0, 0) = Figure::new(KING, WHITE);
    let movies: HashSet<Point> = data_holder.generate_white_movies().iter().map(|m| m.to).collect();

    assert_eq!(movies, expected_movies);
}

#[test]
fn test_generate_king_movies_with_friend_figure() {
    let mut expected_movies = HashSet::new();

    expected_movies.insert(Point::new(1, 0));
    expected_movies.insert(Point::new(0, 1));

    let mut data_holder = DataHolder::new();
    *data_holder.board.cell_mut(0, 0) = Figure::new(KING, WHITE);
    *data_holder.board.cell_mut(1, 1) = Figure::new(PAWN, WHITE);
    let movies: HashSet<Point> = data_holder.generate_figure_movies(0, 0).iter().map(|m| m.to).collect();

    assert_eq!(movies, expected_movies);
}