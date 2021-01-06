use crate::board_controller::BoardController;
use crate::figure::{Figure};
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