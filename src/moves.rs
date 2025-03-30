use crate::pieces;

#[derive(Debug)]
pub enum Outcome {
    Continue,
    Draw,
    Win,
}

#[derive(Debug)]
pub enum Error {
    InvalidNotation,
    InvalidMove,
    MoveIntoCheck,
    AmbiguousMove,
    InvalidCapture,
    InvalidCheck,
    InvalidPromotion,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidNotation => write!(f, "Invalid notation syntax!"),
            Error::InvalidMove => write!(f, "Invalid move!"),
            Error::MoveIntoCheck => write!(f, "That move puts your king in danger!"),
            Error::AmbiguousMove =>
                write!(
                    f,
                    "Multiple pieces can make that move! Please disambiguate by providing a file, rank, or both."
                ),
            Error::InvalidCapture => write!(f, "That move is not a capture! Please omit the 'x'."),
            Error::InvalidCheck =>
                write!(f, "That move is not a check! Please omit the '+' or '#'."),
            Error::InvalidPromotion =>
                write!(
                    f,
                    "Invalid promotion! Make sure the pawn is moving into the last rank and you specify a piece to promote into."
                ),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move {
    pub piece_type: pieces::Piece,
    pub src_file: i8,
    pub src_rank: i8,
    pub dst_file: i8,
    pub dst_rank: i8,
    pub promotion_piece_type: Option<pieces::Piece>,
}

impl Move {
    pub fn new(
        piece_type: pieces::Piece,
        src_file: i8,
        src_rank: i8,
        dst_file: i8,
        dst_rank: i8
    ) -> Self {
        Self {
            piece_type,
            src_file,
            src_rank,
            dst_file,
            dst_rank,
            promotion_piece_type: None,
        }
    }
}
