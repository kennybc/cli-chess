use crate::pieces;
use crate::board;

pub struct Empty {}

impl pieces::Piece for Empty {
    fn get_type(&self) -> pieces::PieceType {
        return pieces::PieceType::Empty;
    }

    fn can_move(&self, _: &board::Board, _: u8, _: u8) -> bool {
        return false;
    }
}

impl std::fmt::Display for Empty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "_")
    }
}
