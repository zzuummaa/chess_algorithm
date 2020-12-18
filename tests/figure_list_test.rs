#![feature(is_sorted)]

extern crate chess_algorithm;

use chess_algorithm::board::*;
use chess_algorithm::figure::*;
use chess_algorithm::figure_list::*;

#[test]
fn test_fill_is_descending_sort() {
    let mut board = ByteBoard::new();
    let mut list = FigureList::new();
    list.fill(&mut board, Color::WHITE);
    assert!(list.iter().is_sorted_by(|a, b| {
        let pa = board.point(*a);
        let pb = board.point(*b);
        Some(pb.weight().cmp(&pa.weight()))
    }));
}
