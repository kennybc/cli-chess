use crate::game;
use crate::pieces;
use crate::board;

pub struct Rook {
    data: pieces::PieceData,
}

impl Rook {
    pub fn new(data: pieces::PieceData) -> Self {
        Rook {
            data,
        }
    }
}

impl pieces::Piece for Rook {
    fn get_player(&self) -> Option<game::Player> {
        return Some(self.data.player);
    }

    fn get_type(&self) -> pieces::PieceType {
        return pieces::PieceType::Rook;
    }

    fn can_move(&self, board: &board::Board, file: i8, rank: i8) -> bool {
        return false;
    }

    fn get_last_move(&self) -> Option<&pieces::PieceMove> {
        return self.data.last_move.as_ref();
    }
}

impl std::fmt::Display for Rook {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.data.player {
            game::Player::White => write!(f, "♖"),
            game::Player::Black => write!(f, "♜"),
        }
    }
}
