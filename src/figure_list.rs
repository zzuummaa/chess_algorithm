#![allow(dead_code)]

use crate::point::*;
use crate::board::*;
use crate::figure::*;
use std::ptr;
use std::fmt;
use std::fmt::{Formatter, Debug};
use core::slice::*;
use crate::movement::Move;
use std::panic::resume_unwind;

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

pub struct LinkedNodeRestoreInfo {
    prev: *mut PointLinkedNode,
    cur: *mut PointLinkedNode,
}

impl Default for LinkedNodeRestoreInfo {
    fn default() -> Self {
        LinkedNodeRestoreInfo { prev: ptr::null_mut(), cur: ptr::null_mut() }
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

#[derive(Copy, Clone)]
pub struct LinkedNodeIterator {
    cur: *mut PointLinkedNode
}

impl Iterator for LinkedNodeIterator {
    type Item = &'static mut PointLinkedNode;

    fn next(&mut self) -> Option<Self::Item> {
        match unsafe { self.cur.as_mut() } {
            None => None,
            Some(cur) => {
                self.cur = cur.next;
                Some(cur)
            }
        }
    }
}

pub struct FigureList {
    pub buffer: [PointLinkedNode; 16],
    pub first: *mut PointLinkedNode,
}

impl FigureList {
    pub fn new(board: &ByteBoard, color: Color) -> Self {
        let mut list = FigureList::default();
        list.fill(board, color);
        return list;
    }

    pub fn fill(&mut self, board: &ByteBoard, color: Color) {
        let mut counter = 0;
        board.cell_iter().for_each(|(p, f)| {
            if f.color() == color {
                self.buffer[counter].point = p;
                counter += 1;
            }
        });
        if counter == 0 {
            self.first = ptr::null_mut();
            return;
        }

        heapsort(&mut self.buffer[0..counter], |a, b| {
            board.point(a.point).weight() > board.point(b.point).weight()
        });
        for i in 0..(counter - 1) {
            self.buffer[i].next = &mut self.buffer[i+1]
        }
        self.first = &mut self.buffer[0];
    }

    pub fn make_move(&mut self, movement: &Move) -> *mut PointLinkedNode {
        let node = self.node_iter()
            .find(|n| n.point == movement.from)
            .unwrap();

        node.point = movement.to;
        return node;
    }

    pub fn unmake_move(movement: &Move, node: *mut PointLinkedNode) {
        unsafe { (*node).point = movement.from; }
    }

    pub fn remove(&mut self, point: Point) -> LinkedNodeRestoreInfo {
        let mut restore_info = LinkedNodeRestoreInfo::default();

        for node in self.node_iter() {
            restore_info.prev = restore_info.cur;
            restore_info.cur = node;
            if node.point == point {
                break;
            }
        }

        if restore_info.cur.is_null() { return restore_info; }

        if restore_info.prev.is_null() {
            self.first = unsafe { (*restore_info.cur).next }
        } else {
            unsafe {
                (*restore_info.prev).next = (*restore_info.cur).next;
            }
        }
        return restore_info;
    }

    pub fn restore(&mut self, restore_info: LinkedNodeRestoreInfo) {
        let cur = match unsafe { restore_info.cur.as_mut() } {
            None => return,
            Some(cur) => cur
        };

        match unsafe { restore_info.prev.as_mut() } {
            None => {
                cur.next = self.first;
                self.first = cur;
            }
            Some(prev) => {
                cur.next = prev.next;
                prev.next = cur;
            }
        }
    }

    pub fn iter(&self) -> PointLinkedNodeIterator {
        PointLinkedNodeIterator { cur : self.first }
    }

    pub fn node_iter(&self) -> LinkedNodeIterator {
        LinkedNodeIterator { cur : self.first }
    }
}

impl Default for FigureList {
    fn default() -> Self {
        FigureList { buffer: [PointLinkedNode::new(); 16], first: ptr::null_mut() }
    }
}

#[allow(unused_variables)]
impl Debug for FigureList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}