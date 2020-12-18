#![allow(dead_code)]

use crate::point::*;
use crate::board::*;
use crate::figure::*;
use std::ptr;
use std::fmt;
use std::fmt::{Formatter, Debug};
use core::slice::*;

#[derive(Copy, Clone)]
pub struct PointArrayNode {
    pub point: Point,
    pub is_present: bool,
}

impl PointArrayNode {
    pub fn new() -> Self {
        PointArrayNode {
            point: Point::default(),
            is_present: false,
        }
    }
}

#[derive(Copy, Clone)]
pub struct PointLinkedNode {
    pub point: Point,
    pub next: *mut PointLinkedNode,
}

impl PointLinkedNode {
    pub fn new() -> Self {
        PointLinkedNode {
            point: Point::default(),
            next: ptr::null_mut()
        }
    }
}

#[derive(Copy, Clone)]
pub struct PointLinkedNodeIterator {
    cur: *mut PointLinkedNode
}

impl Iterator for PointLinkedNodeIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match unsafe { self.cur.as_ref() } {
            None => None,
            Some(e) => {
                self.cur = e.next;
                Some(e.point)
            }
        }
    }
}

pub struct FigureList {
    pub buffer: [PointLinkedNode; 16],
    first: *mut PointLinkedNode,
}

impl FigureList {
    pub fn new() -> Self {
        FigureList { buffer: [PointLinkedNode::new(); 16], first: ptr::null_mut() }
    }

    pub fn fill(&mut self, board: &mut ByteBoard, color: Color) {
        let mut counter = 0;
        board.cell_iter().for_each(|(p, f)| {
            if f.color() == color {
                self.buffer[counter].point = p;
                counter += 1;
            }
        });
        heapsort(&mut self.buffer[0..counter], |a, b| {
            board.point(a.point).weight() > board.point(b.point).weight()
        });
        for i in 0..(counter - 1) {
            self.buffer[i].next = &mut self.buffer[i+1]
        }
        self.first = &mut self.buffer[0];
    }

    pub fn iter(&self) -> PointLinkedNodeIterator {
        PointLinkedNodeIterator { cur : self.first }
    }
}

#[allow(unused_variables)]
impl Debug for FigureList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}