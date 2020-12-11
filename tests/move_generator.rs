extern crate chess_algorithm;
use chess_algorithm::list::*;
use generic_array::typenum::*;
use generic_array::GenericArray;
use std::mem;

struct TestStruct(u8, u8);

#[test]
fn test_simple() {
    let foo = StaticList::<TestStruct, U2> { data: GenericArray::default() };
    println!("foo size: {}", mem::size_of_val(&foo));
}
