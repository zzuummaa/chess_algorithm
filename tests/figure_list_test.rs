extern crate chess_algorithm;

use std::cmp::Ordering;
use chess_algorithm::board::*;
use chess_algorithm::figure::*;
use chess_algorithm::figure_list::*;
use chess_algorithm::figure::Color::{WHITE, NONE};

use chess_algorithm::point::Point;

#[test]
fn test_fill_is_descending_sort() {
    let board = ByteBoard::default();
    let list = FigurePointerList::new(&board, WHITE);
    assert!(list.iter().is_sorted_by(|a, b| {
        let pa = board.point(*a);
        let pb = board.point(*b);
        pb.weight() <= pa.weight()
    }));
}

#[test]
fn test_several_iterators() {
    let mut board = ByteBoard::default();
    let mut list = FigurePointerList::new(&board, WHITE);

    // let movement = Move { from: Point::new(1, 1), to: Point::new(1, 2) };
    let point = Point::new(2, 1);

    // board.make_move(&movement);
    // let movement_info = list.make_move(&movement);
    let removed_figure = *board.point(point);
    *board.point_mut(point) = Figure::new(Rank::NONE, NONE, false);
    let mut cursor = list.node_iter().skip_while(|lnc| lnc.point() != point).next().unwrap();
    cursor.remove();

    assert_eq!(list.node_iter().count(), 15);
    assert_eq!(list.iter().map(|p| board.point(p).weight()).sum::<i32>(), 130000);

    // FigureList::unmake_move(&movement, movement_info);
    cursor.restore();
    *board.point_mut(point) = removed_figure;

    assert_eq!(list.node_iter().count(), 16);
    assert_eq!(list.iter().map(|p| board.point(p).weight()).sum::<i32>(), 131000);
}