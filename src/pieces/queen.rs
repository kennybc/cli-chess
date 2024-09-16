use crate::game;
use crate::pieces;
use crate::board;

pub struct Queen {
    pub pos: pieces::PiecePosition,
    pub last_move: Option<pieces::PieceMove>,
}

impl pieces::Piece for Queen {
    fn get_type(&self) -> pieces::PieceType {
        return pieces::PieceType::Queen;
    }

    fn can_move(&self, board: &board::Board, pos: pieces::PiecePosition) -> bool {
        return false;
    }

    fn get_last_move(&self) -> Option<&pieces::PieceMove> {
        return self.last_move.as_ref();
    }
}

impl std::fmt::Display for Queen {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.pos.player {
            game::Player::White => write!(f, "♕"),
            game::Player::Black => write!(f, "♛"),
        }
    }
}
