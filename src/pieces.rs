use crate::game;

#[derive(Debug, Copy, Clone)]
pub enum Piece {
    Empty,
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl Into<char> for Piece {
    fn into(self) -> char {
        self as u8 as char
    }
}

impl From<char> for Piece {
    fn from(c: char) -> Piece {
        match c {
            'k' => Piece::King,
            'q' => Piece::Queen,
            'r' => Piece::Rook,
            'b' => Piece::Bishop,
            'n' => Piece::Knight,
            _ => Piece::Pawn,
        }
    }
}
