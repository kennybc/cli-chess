use crate::game;
use crate::pieces;
use crate::board;

pub struct King {
    pub pos: pieces::Position,
}

impl pieces::Piece for King {
    fn get_type(&self) -> pieces::PieceType {
        return pieces::PieceType::King;
    }

    fn can_move(&self, board: board::Board, file: u8, rank: u8) -> bool {
        return false;
    }
}

impl std::fmt::Display for King {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.pos.player {
            game::Player::White => write!(f, "♔"),
            game::Player::Black => write!(f, "♚"),
        }
    }
}
