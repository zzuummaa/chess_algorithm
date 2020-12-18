extern crate chess_algorithm;
use chess_algorithm::movement::*;

#[test]
fn test_move_list_default() {
    println!("{:?}", MoveList::default())
}