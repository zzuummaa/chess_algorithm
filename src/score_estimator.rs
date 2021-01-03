use crate::board::ByteBoard;
use crate::figure_list::{FigureList, LinkedNodeRestoreInfo};
use crate::movement::{MoveList, MoveGenerator};
use crate::figure::{Color, W_INFINITY};
use crate::point::Point;

pub struct ScoreEstimator {
    pub board: ByteBoard,
}

impl ScoreEstimator {
    pub fn new(board: &ByteBoard) -> Self {
        ScoreEstimator {
            board: *board
        }
    }

    pub fn min_max_simple(&mut self, depth: i32, friend_list: &mut FigureList, enemy_list: &mut FigureList, enemy_color: Color) -> i32 {
        if depth <= 0 {
            return self.evaluate_score(friend_list, enemy_list);
        }

        // unsafe { println!("{:?}", (*friend_list.first).point); }
        let move_list = MoveList::new(&MoveGenerator::new(&self.board, friend_list));
        // unsafe { println!("{:?}", (*friend_list.first).point); }

        let mut score = - W_INFINITY;
        for movement in move_list.iter() {
            let to_figure = self.board.make_move(movement);
            let figure_list_from_node = friend_list.make_move(movement);

            let mut figure_list_to_node = LinkedNodeRestoreInfo::default();
            if to_figure.color() == enemy_color {
                figure_list_to_node = enemy_list.remove(movement.to);
            }

            let cur_score = - self.min_max_simple(depth - 1, enemy_list, friend_list, enemy_color.invert());
            if cur_score > score { score = cur_score; }

            self.board.unmake_move(movement, to_figure);
            FigureList::unmake_move(movement, figure_list_from_node);

            if to_figure.color() == enemy_color {
                enemy_list.restore(figure_list_to_node);
            }
        }

        return score;
    }

    fn evaluate_score(&self, friend_list: &mut FigureList, enemy_list: &mut FigureList) -> i32 {
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