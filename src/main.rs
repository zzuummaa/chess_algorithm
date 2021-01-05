#![feature(exclusive_range_pattern)]

use std::io;
use std::io::Write;

use chess_algorithm::board::ByteBoard;
use chess_algorithm::figure::Color::{BLACK, WHITE};
use chess_algorithm::figure::Color;
use chess_algorithm::figure::Rank::{KING, NONE};
use chess_algorithm::figure_list::FigurePointerList;
use chess_algorithm::movement::{Move, MoveGenerator, MoveList};
use chess_algorithm::point::Point;
use chess_algorithm::score_estimator::ScoreEstimator;

fn is_king_taken(board: &ByteBoard, list: &FigurePointerList) -> bool {
    list.iter().find(|p| board.point(*p).rank() == KING).is_none()
}

fn generate_move(score_estimator: &mut ScoreEstimator, friend_list: &mut FigurePointerList, enemy_list: &mut FigurePointerList, friend_color: Color, depth: i32) -> Option<Move> {
    score_estimator
        .min_max_simple(depth, friend_list, enemy_list, friend_color).1
        .or_else(|| {
            println!("Movements unavailable. Likely it's draw...");
            return None;
        })
}

fn sub_char(a: char, b: char) -> i8 {
    a as i8 - b as i8
}

fn make_move(board: &mut ByteBoard, friend_list: &mut FigurePointerList, enemy_list: &mut FigurePointerList, movement: &Move) {
    friend_list.make_move(movement);
    let figure = board.make_move(movement);
    if figure.rank() != NONE { enemy_list.remove(movement.to); }
}

fn main() {
    println!("===================================");
    println!("= Chess algorithm console version =");
    println!("===================================");
    println!();

    let mut user_input = String::new();
    let player_color: Color;
    let depth = 5;

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

    let mut score_estimator = ScoreEstimator::new(&ByteBoard::default());
    let mut algorithm_list = FigurePointerList::new(&score_estimator.board, algorithm_color);
    let mut player_list = FigurePointerList::new(&score_estimator.board, player_color);

    if algorithm_color == WHITE {
        let movement = generate_move(&mut score_estimator, &mut algorithm_list, &mut player_list, algorithm_color, depth);
        if movement.is_none() { return; }
        make_move(&mut score_estimator.board, &mut algorithm_list, &mut player_list, &movement.unwrap());
    }

    println!();
    println!("===================================");
    println!("=         Game started!           =");
    println!("===================================");

    loop {
        println!();
        println!("{}", &score_estimator.board);

        if is_king_taken(&score_estimator.board, &algorithm_list) {
            println!("===================================");
            println!("=    Chess algorithm is win!      =");
            println!("===================================");
            break;
        }

        if is_king_taken(&score_estimator.board, &algorithm_list) {
            println!("===================================");
            println!("=         You are win!            =");
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

            let f = score_estimator.board.point(m.from);
            if f.color() != player_color { continue }

            let move_generator = MoveGenerator::new(&score_estimator.board, &player_list);
            let mut move_list = MoveList::default();
            move_generator.fill_for_figure(m.from, &mut move_list);

            if move_list.iter().find(|it| **it == m).is_some() {
                break m;
            }
        };

        make_move(&mut score_estimator.board, &mut player_list, &mut algorithm_list, &player_move);

        let algorithm_move = generate_move(&mut score_estimator, &mut algorithm_list, &mut player_list, algorithm_color, depth);
        if algorithm_move.is_none() { return; }
        make_move(&mut score_estimator.board, &mut algorithm_list, &mut player_list, &algorithm_move.unwrap());

        println!("your move: {}", player_move);
        println!("algorithm move: {}", algorithm_move.unwrap());
        println!();
    }
}
