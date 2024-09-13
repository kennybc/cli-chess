use crate::game;

mod pawn;

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

pub fn can_move(
    player: game::Player,
    piece: Piece,
    src_file: u8,
    src_rank: u8,
    dst_file: u8,
    dst_rank: u8
) -> bool {
    match piece {
        Piece::King => false,
        Piece::Queen => false,
        Piece::Rook => false,
        Piece::Bishop => false,
        Piece::Knight => false,
        Piece::Pawn => pawn::can_move(player, src_file, src_rank, dst_file, dst_rank),
        Piece::Empty => false,
    }
}
