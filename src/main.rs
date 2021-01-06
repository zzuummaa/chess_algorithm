use std::io;
use std::io::Write;

use chess_algorithm::board::ByteBoard;
use chess_algorithm::figure::Color::{BLACK, WHITE};
use chess_algorithm::figure::Color;
use chess_algorithm::movement::Move;
use chess_algorithm::score_estimator::BoardDataHolder;
use std::time::Instant;

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

            let m = match Move::from_string(&user_input.trim_end().to_lowercase()) {
                None => continue,
                Some(m) => m
            };

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
