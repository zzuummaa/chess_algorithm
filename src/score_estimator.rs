use crate::board::ByteBoard;
use crate::figure_list::{FigurePointerList, LinkedNodeRestoreInfo};
use crate::movement::{MoveList, MoveGenerator, Move};
use crate::figure::{Color, W_INFINITY};

pub struct ScoreEstimator {
    pub board: ByteBoard,
}

impl ScoreEstimator {
    pub fn new(board: &ByteBoard) -> Self {
        ScoreEstimator {
            board: *board
        }
    }

    pub fn min_max_simple(&mut self, depth: i32, friend_list: &mut FigurePointerList, enemy_list: &mut FigurePointerList, friend_color: Color) -> (i32, Option<Move>) {
        if depth <= 0 {
            return (self.evaluate_score(friend_list, enemy_list), None);
        }

        // println!("depth: {}, friend_color: {:?}", depth, friend_color);
        // println!("{}", self.board);
        // println!();

        let enemy_color = friend_color.invert();

        // unsafe { println!("{:?}", (*friend_list.first).point); }
        let move_list = MoveList::new(&MoveGenerator::new(&self.board, friend_list));
        // unsafe { println!("{:?}", (*friend_list.first).point); }

        let mut best_score = - W_INFINITY;
        let mut best_move: Option<Move> = None;
        for movement in move_list.iter() {
            let to_figure = self.board.make_move(movement);
            let friend_list_restore_info = friend_list.make_move(movement);

            let mut enemy_list_restore_info = LinkedNodeRestoreInfo::default();
            if to_figure.color() == enemy_color {
                enemy_list_restore_info =
                    enemy_list.remove(movement.to);
                // println!("eaten {} -> {}", self.board.point(movement.to), to_figure);
            }

            // match enemy_list.iter().find(|p| self.board.point(*p).rank() == NONE) {
            //     None => {}
            //     Some(p) => {
            //         unreachable!("{}", p)
            //     }
            // }

            let cur_score = -self.min_max_simple(depth - 1, enemy_list, friend_list, enemy_color).0;
            if cur_score > best_score {
                best_score = cur_score;
                best_move = Some(*movement);
            }

            self.board.unmake_move(movement, to_figure);
            friend_list.unmake_move(movement, friend_list_restore_info);

            if to_figure.color() == enemy_color {
                enemy_list.restore(enemy_list_restore_info);
                // enemy_list.restore(movement.to);
            }
        }

        return (best_score, best_move);
    }

    fn evaluate_score(&self, friend_list: &FigurePointerList, enemy_list: &FigurePointerList) -> i32 {
        let friend_score: i32 = friend_list.iter()
            .map(|p| self.board.point(p).weight())
            .sum();
        // println!("friend_score: {}", friend_score);

        let enemy_score: i32 = enemy_list.iter()
            .map(|p| self.board.point(p).weight())
            .sum();
        // println!("enemy_score: {}", enemy_score);

        friend_score - enemy_score
    }
}