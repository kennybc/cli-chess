use crate::game;
use crate::board;

pub mod empty;
pub mod king;
pub mod queen;
pub mod rook;
pub mod bishop;
pub mod knight;
pub mod pawn;

#[derive(Debug, Copy, Clone)]
pub enum PieceType {
    Empty,
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

pub trait Piece: std::fmt::Display {
    fn get_type(&self) -> PieceType;
    fn can_move(&self, board: board::Board, file: u8, rank: u8) -> bool;
}

pub struct Position {
    pub player: game::Player,
    pub file: u8,
    pub rank: u8,
}

impl Into<char> for PieceType {
    fn into(self) -> char {
        self as u8 as char
    }
}

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
