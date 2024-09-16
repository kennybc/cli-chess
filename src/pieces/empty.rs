use crate::pieces;
use crate::board;

pub struct Empty {}

impl pieces::Piece for Empty {
    fn get_type(&self) -> pieces::PieceType {
        return pieces::PieceType::Empty;
    }

    fn can_move(&self, _: &board::Board, _: pieces::PiecePosition) -> bool {
        return false;
    }

    fn get_last_move(&self) -> Option<&pieces::PieceMove> {
        return None;
    }
}

impl std::fmt::Display for Empty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "_")
    }
}