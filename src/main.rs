use std::io;
use std::io::Write;

use chess_algorithm::board::ByteBoard;
use chess_algorithm::figure::Color::{BLACK, WHITE};
use chess_algorithm::figure::{Color, W_INFINITY};
use chess_algorithm::movement::Move;
use chess_algorithm::board_controller::{BoardDataHolder, BoardController};
use std::time::Instant;

trait MoveSource {
    fn position_counter(&self) -> i32;
    fn next(&mut self, controller: &mut BoardController) -> Option<Move>;
}

#[derive(Default)]
struct ConsoleMoveSource {
    user_input: String
}

impl MoveSource for ConsoleMoveSource {
    fn position_counter(&self) -> i32 {
        0
    }

    fn next(&mut self, controller: &mut BoardController<'_>) -> Option<Move> {
        if controller.friend_movies().len() == 0 { return None }

        println!();
        loop {
            print!("Write {:?} move (a1b2):", controller.friend_color());
            io::stdout().flush().unwrap();

            self.user_input.clear();
            io::stdin()
                .read_line(&mut self.user_input)
                .expect("Failed to read line");

            let m = match Move::from_string(&self.user_input.trim_end().to_lowercase()) {
                None => continue,
                Some(m) => m
            };

            if controller.is_valid_move(&m) {
                break Some(m);
            }
        }
    }
}

#[derive(Default)]
struct SimpleMinMaxMoveSource {
    position_counter: i32
}

impl MoveSource for SimpleMinMaxMoveSource {
    fn position_counter(&self) -> i32 {
        self.position_counter
    }

    fn next(&mut self, controller: &mut BoardController<'_>) -> Option<Move> {
        let movement = controller.min_max_simple(5).1;
        self.position_counter = controller.position_counter;
        return movement;
    }
}

#[derive(Default)]
struct AlphaBettaMoveSource {
    position_counter: i32
}

impl MoveSource for AlphaBettaMoveSource {
    fn position_counter(&self) -> i32 {
        self.position_counter
    }

    fn next(&mut self, controller: &mut BoardController<'_>) -> Option<Move> {
        let movement = controller.alpha_betta(8, - W_INFINITY, W_INFINITY).1;
        self.position_counter = controller.position_counter;
        return movement;
    }
}

fn read_move_source(color: Color) -> Box<dyn MoveSource> {
    loop {
        print!("Type source for {:?} side: ", color);
        io::stdout().flush().unwrap();

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        match user_input.trim().parse::<i32>() {
            Ok(n) => {
                match n {
                    1 => break Box::new(ConsoleMoveSource::default()),
                    2 => break Box::new(SimpleMinMaxMoveSource::default()),
                    3 => break Box::new(AlphaBettaMoveSource::default()),
                    _ => {}
                }
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    }
}

fn main() {
    println!("===================================");
    println!("= Chess algorithm console version =");
    println!("===================================");
    println!();
    println!("Available move sources:");
    println!("1: Console gamer");
    println!("2: Simple min-max algorithm");
    println!("3: Alpha-betta algorithm");
    println!();

    let mut white_source: Box<dyn MoveSource> = read_move_source(WHITE);
    let mut black_source: Box<dyn MoveSource> = read_move_source(BLACK);

    let mut board_data_holder = BoardDataHolder::new(&ByteBoard::default());

    println!();
    println!("===================================");
    println!("=         Game started!           =");
    println!("===================================");
    println!();
    println!("{}", &board_data_holder.board);

    loop {
        let timer = Instant::now();
        let white_move = white_source.next(&mut board_data_holder.controller(WHITE));
        if white_move.is_none() {
            println!("White movements unavailable. Likely it's draw...");
            return;
        }
        board_data_holder.controller(WHITE).make_move(&white_move.unwrap());
        println!();
        println!("{}", &board_data_holder.board);
        println!("white move: {}, {} sec, {} mln positions", white_move.unwrap(), timer.elapsed().as_secs_f32(), white_source.position_counter() as f32 / 1000_000f32);

        if !board_data_holder.controller(BLACK).is_king_alive() {
            println!();
            println!("===================================");
            println!("=      White side is win!         =");
            println!("===================================");
            break;
        }

        let timer = Instant::now();
        let black_move = black_source.next(&mut board_data_holder.controller(BLACK));
        if black_move.is_none() {
            println!("Black movements unavailable. Likely it's draw...");
            return;
        }
        board_data_holder.controller(BLACK).make_move(&black_move.unwrap());
        println!();
        println!("{}", &board_data_holder.board);
        println!("black move: {}, {} sec, {} mln positions", black_move.unwrap(), timer.elapsed().as_secs_f32(), black_source.position_counter() as f32 / 1000_000f32);

        if !board_data_holder.controller(WHITE).is_king_alive() {
            println!();
            println!("===================================");
            println!("=       Black side is win!        =");
            println!("===================================");
            break;
        }
    }
}
