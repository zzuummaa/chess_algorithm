use crate::board::ByteBoard;
use crate::figure::Color::{BLACK, WHITE};
use crate::figure::Rank::{KING, QUEEN};
use crate::figure::{Color, Figure};
use crate::figure_list::{FigurePointList, LinkedNodeCursor};
use crate::movement::MoveType::SIMPLE;
use crate::movement::{Move, MoveGenerator, MoveList, MoveType};
use crate::point::Point;

pub struct BoardDataHolder {
    // TODO remove pub for preventing board changes
    pub board: ByteBoard,
    pub white_list: FigurePointList,
    pub black_list: FigurePointList,
}

pub struct BoardController<'a> {
    pub(crate) board: &'a mut ByteBoard,
    pub(crate) friend_list: &'a mut FigurePointList,
    pub(crate) enemy_list: &'a mut FigurePointList,
    pub(crate) friend_color: Color,
    pub(crate) enemy_color: Color,
    pub position_counter: i32
}

#[derive(Default)]
pub struct PointInfo {
    pub figure: Figure,
    pub point: Point,
    pub cursor: LinkedNodeCursor,
}

impl PointInfo {
    pub fn new(point: &Point, board_controller: &mut BoardController) -> Self {
        let mut info = PointInfo::default();
        info.point = *point;

        info.figure = *board_controller.board.point(*point);

        let node_iter =
            if info.figure.color() == board_controller.friend_color {
                board_controller.friend_list.node_iter()
            } else if info.figure.color() == board_controller.enemy_color {
                board_controller.enemy_list.node_iter()
            } else {
                return info;
            };

        info.cursor = node_iter
            .skip_while(|lnc| lnc.point() != *point)
            .next()
            .expect("Point should be in list");

        return info;
    }
}

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
        if movement.from.x() > 7 || movement.from.x() < 0
        || movement.from.y() > 7 || movement.from.y() < 0 {
            return false;
        }
        if self.board.point(movement.from).color() != self.friend_color {
            return false;
        }
        let move_list = self.point_movies(movement.from);
        move_list.iter().find(|m| **m == *movement).is_some()
    }

    pub fn make_move(&mut self, movement: &Move) -> (PointInfo, PointInfo) {
        let mut from_info = PointInfo::new(&movement.from, self);
        let mut to_info = PointInfo::new(&movement.to, self);

        match movement.m_type {
            MoveType::SIMPLE => {
                from_info.cursor.point_set(movement.to);
                if self.board.point_mut(movement.to).color() == self.enemy_color {
                    to_info.cursor.remove();
                }
                *self.board.point_mut(movement.to) = *self.board.point_mut(movement.from);
                *self.board.point_mut(movement.from) = Figure::empty();

            }
            MoveType::SWAP => {
                from_info.cursor.point_set(movement.to);
                to_info.cursor.point_set(movement.from);
                self.board.swap(movement.from, movement.to);
            }
            MoveType::TRANSFORM => {
                from_info.cursor.point_set(movement.to);
                let f = *self.board.point(movement.from);
                *self.board.point_mut(movement.from) = Figure::empty();
                *self.board.point_mut(movement.to) = Figure::new(QUEEN, f.color(), false);
            }
        }

        (from_info, to_info)
    }

    pub fn unmake_move(&mut self, mut move_info: (PointInfo, PointInfo)) {
        move_info.0.cursor.restore();
        move_info.1.cursor.restore();
        move_info.0.cursor.point_set(move_info.0.point);
        move_info.1.cursor.point_set(move_info.1.point);
        *self.board.point_mut(move_info.0.point) = move_info.0.figure;
        *self.board.point_mut(move_info.1.point) = move_info.1.figure;
    }

    pub fn pass_move_to_enemy(&mut self) {
        std::mem::swap(&mut self.friend_list, &mut self.enemy_list);
        std::mem::swap(&mut self.friend_color, &mut self.enemy_color);
    }

    pub fn is_king_alive(&self) -> bool {
        self.friend_list.iter().find(|p| {
            let f = self.board.point(*p);
            f.rank() == KING && f.color() == self.friend_color
        }).is_some()
    }

    pub fn find_king_eat_move<'b>(&self, move_list: &'b MoveList) -> Option<&'b Move> {
        move_list.iter()
            .filter(|m| m.m_type == SIMPLE)
            .find(|m| {
                let f = self.board.point(m.to);
                f.rank() == KING && f.color() == self.enemy_color
            })
    }
}

impl BoardDataHolder {
    pub fn new(board: &ByteBoard) -> Self {
        BoardDataHolder {
            board: *board,
            white_list: FigurePointList::new(board, WHITE),
            black_list: FigurePointList::new(board, BLACK)
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