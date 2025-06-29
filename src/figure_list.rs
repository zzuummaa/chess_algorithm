#![allow(dead_code)]

use core::slice::*;
use std::collections::linked_list::CursorMut;
use std::collections::LinkedList;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::iter::FilterMap;
use std::ptr;

use crate::board::*;
use crate::figure::*;
use crate::movement::Move;
use crate::point::*;
use std::ptr::null_mut;

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

pub struct LinkedNodeCursor {
    first: *mut *mut PointLinkedNode,
    prev: *mut PointLinkedNode,
    pub(crate) cur: *mut PointLinkedNode
}

impl Default for LinkedNodeCursor {
    fn default() -> Self {
        LinkedNodeCursor {
            first: null_mut(),
            prev: null_mut(),
            cur: null_mut()
        }
    }
}

impl LinkedNodeCursor {
    pub fn remove(&mut self) {
        let cur = unsafe { self.cur.as_ref() }.unwrap();
        match unsafe { self.prev.as_mut() } {
            None => {
                let first = unsafe { self.first.as_mut() }.unwrap();
                *first = cur.next;
            }
            Some(prev) => {
                prev.next = cur.next;
            }
        }
    }

    pub fn restore(&mut self) {
        match unsafe { self.prev.as_mut() } {
            None => {
                match unsafe { self.first.as_mut() } {
                    None => return,
                    Some(first) => *first = self.cur,
                }
            }
            Some(prev) => {
                prev.next = self.cur;
            }
        }
    }

    pub fn point_set(&mut self, point: Point) {
        match unsafe { self.cur.as_mut() } {
            None => {}
            Some(cur) => cur.point = point,
        }
    }

    pub fn point(&self) -> Point {
        let cur = unsafe { self.cur.as_ref() }.unwrap();
        cur.point
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
    first: *mut *mut PointLinkedNode,
    prev: *mut PointLinkedNode,
    cur: *mut PointLinkedNode
}

impl Iterator for LinkedNodeIterator {
    type Item = LinkedNodeCursor;

    fn next(&mut self) -> Option<Self::Item> {
        match unsafe { self.cur.as_mut() } {
            None => None,
            Some(cur) => {
                let cursor = LinkedNodeCursor {
                    first: self.first,
                    prev: self.prev,
                    cur: self.cur
                };
                self.prev = self.cur;
                self.cur = cur.next;
                Some(cursor)
            }
        }
    }
}

pub struct FigureArrayList {
    buffer: [PointArrayNode; 16]
}

impl FigureArrayList {
    pub fn new(board: &ByteBoard, color: Color) -> Self {
        let mut list = FigureArrayList::default();
        list.fill(board, color);
        return list;
    }

    pub fn fill(&mut self, board: &ByteBoard, color: Color) {
        board.cell_iter()
            .filter_map(|(p, f)| if f.color() == color { Some(p) } else { None })
            .enumerate()
            .for_each(|(i, p)| self.buffer[i] = PointArrayNode{ point: p, is_present: true });

        self.buffer.sort_by(|a, b| {
            board.point(b.point).weight().cmp(&board.point(a.point).weight())
        });
    }

    fn find(&self, point: Point, is_present: bool) -> Option<usize> {
        self.buffer.iter().position(|pa| {
            pa.is_present == is_present && pa.point == point
        })
    }

    pub fn make_move(&mut self, movement: &Move) {
        match self.find(movement.from, true) {
            None => unreachable!(),
            Some(i) => {
                self.buffer[i].point = movement.to;
            }
        }
    }

    pub fn unmake_move(&mut self, movement: &Move) {
        match self.find(movement.to, true) {
            None => unreachable!(),
            Some(i) => {
                self.buffer[i].point = movement.from
            }
        }
    }

    pub fn remove(&mut self, point: Point) {
        match self.find(point, true) {
            None => unreachable!(),
            Some(i) => {
                self.buffer[i].is_present = false
            }
        }
    }

    pub fn restore(&mut self, point: Point) {
        match self.find(point, false) {
            None => unreachable!(),
            Some(i) => {
                self.buffer[i].is_present = true
            }
        }
    }

    pub fn iter(&self) -> FilterMap<Iter<'_, PointArrayNode>, fn(&'_ PointArrayNode) -> Option<Point>> {
        self.buffer.iter().filter_map(|pa| if pa.is_present { Some(pa.point) } else { None } )
    }
}

impl Default for FigureArrayList {
    fn default() -> Self {
        FigureArrayList { buffer: [PointArrayNode::new(); 16] }
    }
}

pub struct FigureLinkedList {
    list: LinkedList<Point>
}

impl FigureLinkedList {
    pub fn new(board: &ByteBoard, color: Color) -> Self {
        let mut list = FigureLinkedList::default();
        list.fill(board, color);
        return list;
    }

    pub fn fill(&mut self, board: &ByteBoard, color: Color) {
        let mut vec: Vec<_> = board.cell_iter()
            .filter_map(|(p, f)| if f.color() == color { Some(p) } else { None })
            .collect();

        vec.sort_by(|a, b| {
            board.point(*b).weight().cmp(&board.point(*a).weight())
        });

        self.list = vec.iter().map(|p| *p).collect();
    }

    fn find(&mut self, point: Point) -> CursorMut<Point> {
        let mut cursor = self.list.cursor_front_mut();

        loop {
            match cursor.current() {
                None => break cursor,
                Some(p) => {
                    if *p == point {
                        break cursor;
                    }
                }
            }
            cursor.move_next();
        }
    }

    pub fn make_move(&mut self, movement: &Move) {
        let mut cursor = self.find(movement.from);
        match cursor.current() {
            Some(p) => {
                *p = movement.to;
            }
            None => unreachable!()
        }
    }

    pub fn unmake_move(&mut self, movement: &Move) {
        let mut cursor = self.find(movement.to);
        match cursor.current() {
            Some(p) => {
                *p = movement.from;
            }
            None => unreachable!()
        }
    }

    pub fn remove(&mut self, point: Point) {
        let mut cursor = self.find(point);
        if cursor.current().is_none() { return; }
        cursor.remove_current();
    }

    pub fn restore(&mut self, point: Point) {
        self.list.push_front(point);
    }

    pub fn iter(&self) -> std::collections::linked_list::Iter<'_, Point> {
        self.list.iter()
    }
}

impl Default for FigureLinkedList {
    fn default() -> Self {
        FigureLinkedList { list: LinkedList::new() }
    }
}

pub struct FigurePointList {
    pub nodes: Box<[PointLinkedNode; 16]>,
    pub first: *mut PointLinkedNode,
}

impl FigurePointList {
    pub fn new(board: &ByteBoard, color: Color) -> Self {
        let mut list = FigurePointList::default();
        list.fill(board, color);
        return list;
    }

    pub fn fill(&mut self, board: &ByteBoard, color: Color) {
        let mut counter = 0;
        board.cell_iter().for_each(|(p, f)| {
            if f.color() == color {
                self.nodes[counter].point = p;
                counter += 1;
            }
        });
        if counter == 0 {
            self.first = ptr::null_mut();
            return;
        }

        self.nodes.sort_by(|a, b| {
            board.point(b.point).weight().cmp(&board.point(a.point).weight())
        });
        for i in 0..(counter - 1) {
            self.nodes[i].next = &mut self.nodes[i+1]
        }
        self.first = &mut self.nodes[0];
    }

    // pub fn make_move(&mut self, movement: &Move) -> LinkedNodeCursor {
    //     let node = self.node_iter()
    //         .find(|n| n.point == movement.from)
    //         .unwrap();
    //
    //     node.point = movement.to;
    //     return LinkedNodeCursor { first: &mut self.first, prev: null_mut(), cur: node };
    // }
    //
    // pub fn unmake_move(&mut self, movement: &Move, node: LinkedNodeCursor) {
    //     unsafe { (*node.cur).point = movement.from; }
    // }
    //
    // pub fn remove(&mut self, point: Point) -> LinkedNodeCursor {
    //     let mut restore_info = LinkedNodeCursor::default();
    //
    //     for node in self.node_iter() {
    //         restore_info.prev = restore_info.cur;
    //         restore_info.cur = node;
    //         if node.point == point {
    //             break;
    //         }
    //     }
    //
    //     if restore_info.cur.is_null() { return restore_info; }
    //
    //     if restore_info.prev.is_null() {
    //         self.first = unsafe { (*restore_info.cur).next }
    //     } else {
    //         unsafe {
    //             (*restore_info.prev).next = (*restore_info.cur).next;
    //         }
    //     }
    //     return restore_info;
    // }
    //
    // pub fn restore(&mut self, restore_info: LinkedNodeCursor) {
    //     let cur = match unsafe { restore_info.cur.as_mut() } {
    //         None => return,
    //         Some(cur) => cur
    //     };
    //
    //     match unsafe { restore_info.prev.as_mut() } {
    //         None => {
    //             cur.next = self.first;
    //             self.first = cur;
    //         }
    //         Some(prev) => {
    //             cur.next = prev.next;
    //             prev.next = cur;
    //         }
    //     }
    // }

    pub fn iter(&self) -> PointLinkedNodeIterator {
        PointLinkedNodeIterator { cur : self.first }
    }

    pub fn node_iter(&mut self) -> LinkedNodeIterator {
        LinkedNodeIterator { first: &mut self.first, prev: null_mut(), cur : self.first }
    }
}

impl Default for FigurePointList {
    fn default() -> Self {
        FigurePointList { nodes: Box::new([PointLinkedNode::new(); 16]), first: ptr::null_mut() }
    }
}

impl Display for FigurePointList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for p in self.iter() {
            write!(f, "{}, ", p)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[allow(unused_variables)]
impl Debug for FigurePointList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}