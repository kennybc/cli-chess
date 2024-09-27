use crate::pieces;
use crate::board;
use crate::game;
use crate::moves;

#[derive(Clone)]
pub struct Empty {}

impl pieces::Piece for Empty {
    fn get_player(&self) -> Option<game::Player> {
        return None;
    }

    fn get_type(&self) -> pieces::PieceType {
        return pieces::PieceType::Empty;
    }

    fn can_attack(&self, _: &board::Board, _: i8, _: i8) -> bool {
        return false;
    }

    fn can_move(&self, _: &board::Board, _: i8, _: i8) -> bool {
        return false;
    }

    fn get_last_move(&self) -> Option<&(i32, moves::PieceMove)> {
        return None;
    }

    fn set_last_move(&mut self, _: i32, _: moves::PieceMove) {}
}

impl std::fmt::Display for Empty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, " ")
    }
}
