use crate::game;
use crate::board;

pub mod empty;
pub mod king;
pub mod queen;
pub mod rook;
pub mod bishop;
pub mod knight;
pub mod pawn;

pub struct PieceMove {
    pub piece_type: PieceType,
    pub src_file: u8,
    pub src_rank: u8,
    pub dst_file: u8,
    pub dst_rank: u8,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    Empty,
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

// allow inference from char (for notation parsing)
impl From<char> for PieceType {
    fn from(c: char) -> PieceType {
        match c {
            'k' => PieceType::King,
            'q' => PieceType::Queen,
            'r' => PieceType::Rook,
            'b' => PieceType::Bishop,
            'n' => PieceType::Knight,
            _ => PieceType::Pawn,
        }
    }
}

pub trait Piece: std::fmt::Display {
    fn get_type(&self) -> PieceType;
    fn can_move(&self, board: &board::Board, file: u8, rank: u8) -> bool;
    fn get_last_move(&self) -> Option<&PieceMove>;
}

pub struct PieceData {
    pub player: game::Player,
    pub file: u8,
    pub rank: u8,
    pub last_move: Option<PieceMove>,
}
