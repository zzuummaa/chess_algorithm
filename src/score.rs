use crate::board_controller::BoardController;
use crate::figure::{Figure, W_INFINITY};
use crate::movement::Move;
use crate::point::Point;

pub fn evaluate_score<T: Fn(Point, Figure) -> i32>(controller: &BoardController, eval_fn: T) -> i32 {
    let friend_score: i32 = controller.friend_list.iter()
        .map(|p| eval_fn(p, *controller.board.point(p)))
        .sum();
    // println!("friend_score: {}", friend_score);

    let enemy_score: i32 = controller.enemy_list.iter()
        .map(|p| eval_fn(p, *controller.board.point(p)))
        .sum();
    // println!("enemy_score: {}", enemy_score);

    friend_score - enemy_score
}

pub fn material_fn(_: Point, f: Figure) -> i32 {
    f.weight()
}

pub fn simple_positional_fn(p: Point, f: Figure) -> i32 {
    (unsafe { std::mem::transmute::<_, u8>(f.color()) } as i32 - 64) + p.y() as i32 * 8 + (8 - p.x() as i32)
}

#[derive(Default)]
pub struct MinMaxSimpleSearch {}

#[derive(Default)]
pub struct AlphaBetaSearch {}

pub trait MoveSearch {
    fn find_best_move(&self, controller: &mut BoardController, depth: i32) -> (i32, Option<Move>);
}

impl MoveSearch for MinMaxSimpleSearch {
    fn find_best_move(&self, controller: &mut BoardController, depth: i32) -> (i32, Option<Move>) {
        min_max_simple(controller, depth)
    }
}

impl MoveSearch for AlphaBetaSearch {
    fn find_best_move(&self, controller: &mut BoardController, depth: i32) -> (i32, Option<Move>) {
        alpha_betta(controller, depth, - W_INFINITY, W_INFINITY)
    }
}

pub fn min_max_simple(controller: &mut BoardController, depth: i32) -> (i32, Option<Move>) {
    if depth <= 0 {
        controller.position_counter += 1;
        return (evaluate_score(controller, |p, f| {
            material_fn(p, f) + simple_positional_fn(p, f)
        }), None);
    }

    // unsafe { println!("{:?}", (*friend_list.first).point); }
    let move_list = controller.friend_movies();

    // if let Some(king_eat_move) = self.find_king_eat_move(&move_list) {
    //     return (W_INFINITY, Some(*king_eat_move));
    // }
    // MoveList::default();
    // unsafe { println!("{:?}", (*friend_list.first).point); }

    let mut best_score = - W_INFINITY;
    let mut best_move: Option<Move> = move_list.iter().next().copied();
    for movement in move_list.iter() {
        let move_info = controller.make_move(movement);
        controller.pass_move_to_enemy();

        // println!("{}", self.board);
        // println!();

        let cur_score = - min_max_simple(controller, depth - 1).0;

        controller.pass_move_to_enemy();
        controller.unmake_move(move_info);

        if cur_score > best_score {
            best_score = cur_score;
            best_move = Some(*movement);
        }

    }

    (best_score, best_move)
}

pub fn alpha_betta(controller: &mut BoardController, depth: i32, mut alpha: i32, betta: i32) -> (i32, Option<Move>) {
    if depth <= 0 {
        controller.position_counter += 1;
        return (evaluate_score(controller, |p, f| {
            material_fn(p, f) + simple_positional_fn(p, f)
        }), None);
    }

    let mut move_list = controller.friend_movies();
    move_list.sort_by(controller.board, simple_positional_fn);

    // if let Some(first_move) = move_list.iter().next() {
    //     let f = self.board.point(first_move.to);
    //     if f.rank() == KING && f.color() == self.enemy_color && first_move.m_type == SIMPLE {
    //         return (W_INFINITY, Some(*first_move));
    //     }
    // }

    let mut best_score = - W_INFINITY;
    let mut best_move: Option<Move> = move_list.iter().next().copied();
    for movement in move_list.iter() {
        let move_info = controller.make_move(movement);
        controller.pass_move_to_enemy();

        let mut cur_score = - alpha_betta(controller, depth - 1, - (alpha + 1), - alpha).0;
        if cur_score > alpha && cur_score < betta {
            cur_score = - alpha_betta(controller, depth - 1, - betta, - alpha).0;
        }

        controller.pass_move_to_enemy();
        controller.unmake_move(move_info);

        if cur_score > best_score {
            best_score = cur_score;
            best_move = Some(*movement);
        }

        if best_score > alpha { alpha = best_score }
        if alpha >= betta { return (alpha, best_move) }
    }

    (best_score, best_move)
}
