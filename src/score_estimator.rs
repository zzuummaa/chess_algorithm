use crate::board::ByteBoard;
use crate::figure_list::{FigureList, PointLinkedNode, LinkedNodeRestoreInfo};
use crate::movement::{MoveList, MoveGenerator};
use crate::figure::Color::{WHITE, BLACK};
use crate::figure::{Color, Figure, W_INFINITY};
use crate::figure::Rank::NONE;

pub struct ScoreEstimator {
    pub board: ByteBoard,
    pub white_list: FigureList,
    pub black_list: FigureList,
}

impl ScoreEstimator {
    pub fn new(board: &ByteBoard) -> Self {
        ScoreEstimator {
            board: *board,
            white_list: FigureList::new(board, WHITE),
            black_list: FigureList::new(board, BLACK)
        }
    }

    pub fn min_max_simple(&mut self, depth: i32, color: Color) -> i32 {
        if depth <= 0 {
            return self.evaluate_score(color);
        }

        let self_p: *mut Self = &mut *self;
        let self_ref = unsafe {self_p.as_mut() }.unwrap();

        let (friend_list, enemy_list, enemy_color) = match color {
            Color::NONE => unreachable!(),
            WHITE => (&mut self.white_list, &mut self.black_list, BLACK),
            BLACK => (&mut self.black_list, &mut self.white_list, WHITE),
        };

        let move_list = MoveList::new(&MoveGenerator::new(&self.board, friend_list));

        let mut score = - W_INFINITY;
        for movement in move_list.iter() {
            let to_figure = self.board.make_move(movement);

            let figure_list_from_node = friend_list.make_move(movement);

            let mut figure_list_to_node = LinkedNodeRestoreInfo::default();
            if to_figure.color() == enemy_color {
                figure_list_to_node = enemy_list.remove(movement.to);
            }

            let cur_score = - self_ref.min_max_simple(depth - 1, enemy_color);
            if cur_score > score { score = cur_score; }

            self.board.unmake_move(movement, to_figure);

            FigureList::unmake_move(movement, figure_list_from_node);

            if to_figure.color() == enemy_color {
                enemy_list.restore(&mut figure_list_to_node);
            }
        }

        return score;
    }

    fn evaluate_score(&mut self, color: Color) -> i32 {
        let white_score: i32 = self.white_list.iter()
            .map(|p| self.board.point(p).weight())
            .sum();

        let black_score: i32 = self.black_list.iter()
            .map(|p| self.board.point(p).weight())
            .sum();

        match color {
            Color::NONE => unreachable!(),
            WHITE => white_score - black_score,
            BLACK => black_score - white_score
        }
    }
}