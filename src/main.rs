use std::io;
use std::io::Write;

use chess_algorithm::board::ByteBoard;
use chess_algorithm::figure::Color::{BLACK, WHITE};
use chess_algorithm::figure::{Color, W_INFINITY};
use chess_algorithm::movement::Move;
use chess_algorithm::board_controller::{BoardDataHolder, BoardController};
use std::time::Instant;
use chess_algorithm::database::{DataBaseInstance, Game, MoveRecord};

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

            let m = match Move::from_string(&self.user_input.trim_end().to_uppercase()) {
                Err(_) => continue,
                Ok(m) => m
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

fn load_board(db_instance: &mut DataBaseInstance) -> (ByteBoard, Game, MoveRecord) {
    loop {
        print!("Load game or start new: ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        if user_input.trim().is_empty() {
            let game = db_instance.add_game(Game::now()).unwrap();
            let record = MoveRecord::new(&game);
            return (
                ByteBoard::default(),
                game,
                record
            );
        }

        let id = match user_input.trim().parse::<i32>() {
            Ok(id) => id,
            Err(e) => {
                println!("{}", e);
                continue
            }
        };

        let game = match db_instance.find_game(id) {
            Ok(g) => g,
            Err(e) => {
                println!("{}", e);
                continue
            }
        };

        let move_records_result = match db_instance.find_moves(&game) {
            Ok(records) => records,
            Err(e) => {
                println!("{}", e);
                continue
            }
        };

        let moves_result = move_records_result.iter()
            .map(|r| r.to_move())
            .collect::<Result<Vec<Move>, _>>();

        let move_records = match moves_result {
            Ok(records) => records,
            Err(e) => {
                println!("{}", e);
                continue
            }
        };

        println!("===================================");
        println!("=           Load game             =");
        println!("===================================");
        println!();

        let mut holder = BoardDataHolder::new(&ByteBoard::default());

        move_records.iter().enumerate().for_each(|r| {
            let c =
                if r.0 % 2 == 0 {
                    WHITE
                } else {
                    BLACK
                };

            let mut controller = holder.controller(c);
            if !controller.is_valid_move(r.1) { unreachable!("{:?} move: {}", c, r.1) };
            controller.make_move(r.1);

            println!();
            println!("{}", &holder.board);
            println!("{:?} move: {}", c, r.1);
        });

        let mut move_records_result = move_records_result;
        let last_record = if move_records_result.is_empty() {
            MoveRecord::new(&game)
        } else {
            move_records_result.remove(move_records_result.len() - 1)
        };

        return (holder.board, game, last_record);
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

    let mut db_instance = DataBaseInstance::default();
    let (board, _, mut move_record) = load_board(&mut db_instance);

    let mut board_data_holder = BoardDataHolder::new(&board);
    drop(board);

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

        move_record = move_record.to_next(&white_move.unwrap());
        db_instance.add_move(&move_record).unwrap();

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

        move_record = move_record.to_next(&black_move.unwrap());
        db_instance.add_move(&move_record).unwrap();

        if !board_data_holder.controller(WHITE).is_king_alive() {
            println!();
            println!("===================================");
            println!("=       Black side is win!        =");
            println!("===================================");
            break;
        }
    }
}
