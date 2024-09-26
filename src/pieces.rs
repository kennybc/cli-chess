use dyn_clone::DynClone;

use crate::game;
use crate::board;
use crate::pieces;

pub mod empty;
pub mod king;
pub mod queen;
pub mod rook;
pub mod bishop;
pub mod knight;
pub mod pawn;

#[derive(Debug)]
pub enum MoveError {
    InvalidNotation,
    InvalidMove,
    MoveIntoCheck,
    AmbiguousMove,
    InvalidCapture,
    InvalidCheck,
    InvalidPromotion,
}

impl std::fmt::Display for pieces::MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            pieces::MoveError::InvalidNotation => write!(f, "Invalid notation syntax!"),
            pieces::MoveError::InvalidMove => write!(f, "Invalid move!"),
            pieces::MoveError::MoveIntoCheck => write!(f, "That move puts your king in danger!"),
            pieces::MoveError::AmbiguousMove =>
                write!(
                    f,
                    "Multiple pieces can make that move! Please disambiguate by providing a file, rank, or both."
                ),
            pieces::MoveError::InvalidCapture =>
                write!(f, "That move is not a capture! Please omit the 'x'."),
            pieces::MoveError::InvalidCheck =>
                write!(f, "That move is not a check! Please omit the '+' or '#'."),
            pieces::MoveError::InvalidPromotion =>
                write!(
                    f,
                    "Invalid promotion! Make sure the pawn is moving into the last rank and you specify a piece to promote into."
                ),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PieceMove {
    pub piece_type: PieceType,
    pub src_file: i8,
    pub src_rank: i8,
    pub dst_file: i8,
    pub dst_rank: i8,
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

pub trait Piece: DynClone + std::fmt::Display {
    fn get_player(&self) -> Option<game::Player>;
    fn get_type(&self) -> PieceType;
    fn can_attack(&self, board: &board::Board, file: i8, rank: i8) -> bool;
    fn can_move(&self, board: &board::Board, file: i8, rank: i8) -> bool;
    fn get_last_move(&self) -> Option<&(i32, PieceMove)>;
    fn set_last_move(&mut self, turn: i32, mv: pieces::PieceMove);
}

#[derive(Clone)]
pub struct PieceData {
    pub player: game::Player,
    pub file: i8,
    pub rank: i8,
    pub last_move: Option<(i32, PieceMove)>,
}
