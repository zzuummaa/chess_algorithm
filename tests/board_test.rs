extern crate chess_algorithm;
use chess_algorithm::board::*;

#[test]
fn test_print_board() {
    println!("{}", ByteBoard::new())
}
