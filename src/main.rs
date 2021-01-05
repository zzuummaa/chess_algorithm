#![feature(exclusive_range_pattern)]

use std::io;
use std::io::Write;

use chess_algorithm::board::ByteBoard;
use chess_algorithm::figure::Color::{BLACK, WHITE};
use chess_algorithm::figure::Color;
use chess_algorithm::movement::Move;
use chess_algorithm::point::Point;
use chess_algorithm::score_estimator::BoardDataHolder;
use std::time::Instant;

fn sub_char(a: char, b: char) -> i8 {
    a as i8 - b as i8
}

fn main() {
    println!("===================================");
    println!("= Chess algorithm console version =");
    println!("===================================");
    println!();

    let mut user_input = String::new();
    let player_color: Color;
    let depth = 6;

    loop {
        print!("Would you like to play white? (y/n): ");
        io::stdout().flush().unwrap();

        user_input.clear();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        player_color = match user_input.trim_end().to_lowercase().as_str() {
            "y" | "yes" => WHITE,
            "n" | "no" => BLACK,
            _ => continue
        };
        break;
    }

    let algorithm_color = player_color.invert();
    let mut board_data_holder = BoardDataHolder::new(&ByteBoard::default());

    println!();
    println!("===================================");
    println!("=         Game started!           =");
    println!("===================================");

    if algorithm_color == WHITE {
        let timer = Instant::now();

        let algorithm_move = board_data_holder
            .controller(algorithm_color)
            .min_max_simple(depth).1;

        if algorithm_move.is_none() { return; }
        board_data_holder.controller(algorithm_color).make_move(&algorithm_move.unwrap());

        println!("algorithm move: {}", algorithm_move.unwrap());
        println!("calc time: {} sec", timer.elapsed().as_secs_f32())
    }

    loop {
        println!();
        println!("{}", &board_data_holder.board);

        if board_data_holder.controller(player_color).is_king_alive() {
            println!();
            println!("===================================");
            println!("=    Chess algorithm is win!      =");
            println!("===================================");
            break;
        }

        println!();
        let player_move = loop {
            print!("Write your move (a1b2):");
            io::stdout().flush().unwrap();

            user_input.clear();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");

            user_input = user_input.trim_end().to_lowercase();

            if user_input.len() != 4 {
                continue;
            }

            let mut m = Move { from: Point::default(), to: Point::default() };
            let parse_count = user_input.char_indices().filter(|c| {
                match c.0 {
                    0..4 => {
                        match c.0 {
                            0 => if c.1 >= 'a' && c.1 <= 'h' { m.from = m.from + Point::new(-sub_char(c.1, 'h'), 0) }
                            1 => if c.1 >= '1' && c.1 <= '8' { m.from = m.from + Point::new(0, sub_char(c.1, '1')) }
                            2 => if c.1 >= 'a' && c.1 <= 'h' { m.to = m.to + Point::new(-sub_char(c.1, 'h'), 0) }
                            3 => if c.1 >= '1' && c.1 <= '8' { m.to = m.to + Point::new(0, sub_char(c.1, '1')) }
                            _ => unreachable!()
                        }
                        true
                    }
                    _ => false
                }
            }).count();

            if parse_count != 4 { continue }

            if board_data_holder.controller(player_color).validate_and_make_move(&m).is_some() {
                break m;
            }
        };
        println!("your move: {}", player_move);

        if board_data_holder.controller(algorithm_color).is_king_alive() {
            println!();
            println!("===================================");
            println!("=         You are win!            =");
            println!("===================================");
            break;
        }

        let timer = Instant::now();
        let algorithm_move = board_data_holder.controller(algorithm_color).min_max_simple(depth).1;
        if algorithm_move.is_none() {
            println!("Movements unavailable. Likely it's draw...");
            return;
        }
        board_data_holder.controller(algorithm_color).make_move(&algorithm_move.unwrap());

        println!("algorithm move: {}", algorithm_move.unwrap());
        println!("calc time: {} sec", timer.elapsed().as_secs_f32())
    }
}
