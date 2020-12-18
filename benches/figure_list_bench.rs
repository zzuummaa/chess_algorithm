#![feature(test)]
extern crate test;
extern crate chess_algorithm;

use test::Bencher;
use chess_algorithm::figure_list::*;

#[bench]
fn bench_array_iterations(b: &mut Bencher) {
    let mut nodes = [PointArrayNode::new(); 18];

    nodes.iter_mut().enumerate().for_each(|e| {
       e.1.is_present = e.0 % 2 == 0
    });

    let mut count = 0;
    b.iter(|| {
        count += nodes.iter().filter(|e| !e.is_present).count()
    });
    println!("{}", count)
}

#[bench]
fn bench_node_iterations(b: &mut Bencher) {
    let mut nodes = [PointLinkedNode::new(); 18];

    for i in (0..nodes.len() - 2).step_by(2) {
        nodes[i].next = &mut nodes[i+2];
    }

    let mut count = 0;
    b.iter(|| {
        count += nodes.iter().count()
    });
    println!("{}", count)
}