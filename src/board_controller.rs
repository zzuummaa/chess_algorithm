use crate::board::ByteBoard;
use crate::figure_list::{FigurePointerList, LinkedNodeRestoreInfo, PointLinkedNode};
use crate::movement::{MoveList, MoveGenerator, Move};
use crate::figure::{Color, W_INFINITY, Figure};
use crate::figure::Color::{WHITE, BLACK};
use crate::point::Point;
use crate::figure::Rank::KING;
use crate::score::*;

pub struct BoardDataHolder {
    // TODO remove pub for preventing board changes
    pub board: ByteBoard,
    white_list: FigurePointerList,
    black_list: FigurePointerList,
}

pub struct BoardController<'a> {
    pub(crate) board: &'a mut ByteBoard,
    pub(crate) friend_list: &'a mut FigurePointerList,
    pub(crate) enemy_list: &'a mut FigurePointerList,
    pub(crate) friend_color: Color,
    pub(crate) enemy_color: Color,
    pub position_counter: i32
}

type MoveInfo = (Figure, *mut PointLinkedNode, LinkedNodeRestoreInfo, Move);

impl<'a> BoardController<'a> {
    #[inline]
    pub fn friend_color(&self) -> Color {
        self.friend_color
    }

    pub fn friend_movies(&self) -> MoveList {
        MoveList::new(&MoveGenerator::new(self.board, self.friend_list))
    }

    pub fn point_movies(&self, point: Point) -> MoveList {
        let mut move_list = MoveList::default();
        MoveGenerator::new(self.board, self.friend_list).fill_for_figure(point, &mut move_list);
        return move_list;
    }

    pub fn is_valid_move(&self, movement: &Move) -> bool {
        let move_list = self.point_movies(movement.from);
        move_list.iter().find(|m| **m == *movement).is_some()
    }

    pub fn make_move(&mut self, movement: &Move) -> MoveInfo {
        let to_figure = self.board.make_move(movement);
        let friend_list_restore_info = self.friend_list.make_move(movement);

        let mut enemy_list_restore_info = LinkedNodeRestoreInfo::default();
        if to_figure.color() == self.enemy_color {
            enemy_list_restore_info = self.enemy_list.remove(movement.to);
        }

        (to_figure, friend_list_restore_info, enemy_list_restore_info, *movement)
    }

    pub fn unmake_move(&mut self, move_info: MoveInfo) {
        self.board.unmake_move(&move_info.3, move_info.0);
        self.friend_list.unmake_move(&move_info.3, move_info.1);

        if move_info.0.color() == self.enemy_color {
            self.enemy_list.restore(move_info.2);
            // enemy_list.restore(movement.to);
        }
    }

    pub fn pass_move_to_enemy(&mut self) {
        std::mem::swap(&mut self.friend_list, &mut self.enemy_list);
        std::mem::swap(&mut self.friend_color, &mut self.enemy_color);
    }

    pub fn min_max_simple(&mut self, depth: i32) -> (i32, Option<Move>) {
        if depth <= 0 {
            self.position_counter += 1;
            return (evaluate_score(self, |p, f| {
                material_fn(p, f) + simple_positional_fn(p, f)
            }), None);
        }

        // unsafe { println!("{:?}", (*friend_list.first).point); }
        let move_list =
            self.friend_movies();
            // MoveList::default();
        // unsafe { println!("{:?}", (*friend_list.first).point); }

        let mut best_score = - W_INFINITY;
        let mut best_move: Option<Move> = None;
        for movement in move_list.iter() {
            let move_info = self.make_move(movement);

            // match enemy_list.iter().find(|p| self.board.point(*p).rank() == NONE) {
            //     None => {}
            //     Some(p) => {
            //         unreachable!("{}", p)
            //     }
            // }

            self.pass_move_to_enemy();
            let cur_score = - self.min_max_simple(depth - 1).0;
            self.pass_move_to_enemy();

            if cur_score > best_score {
                best_score = cur_score;
                best_move = Some(*movement);
            }

            self.unmake_move(move_info);
        }

        return (best_score, best_move);
    }

    pub fn is_king_alive(&self) -> bool {
        self.friend_list.iter().find(|p| {
            let f = self.board.point(*p);
            f.rank() == KING && f.color() == self.friend_color
        }).is_some()
    }
}

impl BoardDataHolder {
    pub fn new(board: &ByteBoard) -> Self {
        BoardDataHolder {
            board: *board,
            white_list: FigurePointerList::new(board, WHITE),
            black_list: FigurePointerList::new(board, BLACK)
        }
    }

    pub fn controller(&mut self, color: Color) -> BoardController {
        match color {
            WHITE =>  {
                BoardController {
                    board: &mut self.board,
                    friend_list: &mut self.white_list,
                    enemy_list: &mut self.black_list,
                    friend_color: WHITE,
                    enemy_color: BLACK,
                    position_counter: 0
                }
            }
            BLACK => {
                BoardController {
                    board: &mut self.board,
                    friend_list: &mut self.black_list,
                    enemy_list: &mut self.white_list,
                    friend_color: BLACK,
                    enemy_color: WHITE,
                    position_counter: 0
                }
            }
            _ => unreachable!(),
        }
    }
}