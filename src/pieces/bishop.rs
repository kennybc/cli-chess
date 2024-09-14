use crate::game;
use crate::pieces;
use crate::board;

pub struct Bishop {
    pub pos: pieces::Position,
}

impl pieces::Piece for Bishop {
    fn get_type(&self) -> pieces::PieceType {
        return pieces::PieceType::Bishop;
    }

    fn can_move(&self, board: &board::Board, file: u8, rank: u8) -> bool {
        return false;
    }
}

impl std::fmt::Display for Bishop {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.pos.player {
            game::Player::White => write!(f, "♗"),
            game::Player::Black => write!(f, "♝"),
        }
    }
}
