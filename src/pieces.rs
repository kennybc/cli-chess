use dyn_clone::DynClone;

use crate::game;
use crate::board;
use crate::moves;

pub mod empty;
pub mod king;
pub mod queen;
pub mod rook;
pub mod bishop;
pub mod knight;
pub mod pawn;

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

pub trait Piece: DynClone + std::fmt::Display {
    fn get_player(&self) -> Option<game::Player>;
    fn get_type(&self) -> PieceType;
    fn can_attack(&self, board: &board::Board, file: i8, rank: i8) -> bool;
    fn can_move(&self, board: &board::Board, file: i8, rank: i8) -> bool;
    fn get_last_move(&self) -> Option<&(i32, moves::PieceMove)>;
    fn set_last_move(&mut self, turn: i32, mv: moves::PieceMove);
}

#[derive(Clone)]
pub struct PieceData {
    pub player: game::Player,
    pub file: i8,
    pub rank: i8,
    pub last_move: Option<(i32, moves::PieceMove)>,
}
