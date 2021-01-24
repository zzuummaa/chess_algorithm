#![feature(sort_internals)]
#![feature(linked_list_cursors)]
#![feature(exclusive_range_pattern)]

#[macro_use]
extern crate enum_display_derive;

pub mod board;
pub mod figure;
pub mod figure_list;
pub mod point;
pub mod movement;
pub mod board_controller;
pub mod game;
pub mod score;
pub mod database;
