mod pawn;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    White = 0,
    Black = 1,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

// allow inference from char (for notation parsing)
impl Piece {
    pub fn from_char(c: char) -> Piece {
        match c {
            'K' => Piece::King,
            'Q' => Piece::Queen,
            'R' => Piece::Rook,
            'B' => Piece::Bishop,
            'N' => Piece::Knight,
            _ => Piece::Pawn,
        }
    }
}
