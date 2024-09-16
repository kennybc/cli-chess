use crate::game;
use crate::pieces;
use crate::board;

pub struct King {
    data: pieces::PieceData,
}

impl King {
    pub fn new(data: pieces::PieceData) -> Self {
        King {
            data,
        }
    }
}

impl pieces::Piece for King {
    fn get_type(&self) -> pieces::PieceType {
        return pieces::PieceType::King;
    }

    fn can_move(&self, board: &board::Board, file: u8, rank: u8) -> bool {
        return false;
    }

    fn get_last_move(&self) -> Option<&pieces::PieceMove> {
        return self.data.last_move.as_ref();
    }
}

impl std::fmt::Display for King {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.data.player {
            game::Player::White => write!(f, "♔"),
            game::Player::Black => write!(f, "♚"),
        }
    }
}
