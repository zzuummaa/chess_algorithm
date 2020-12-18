extern crate chess_algorithm;
use chess_algorithm::board::*;

#[test]
fn test_print_board() {
    let board = ByteBoard::new();
    println!("{}", board)
}