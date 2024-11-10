use crate::pieces;

#[derive(Debug)]
pub enum MoveOutcome {
    Continue,
    Draw,
    Win,
}

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

impl std::fmt::Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MoveError::InvalidNotation => write!(f, "Invalid notation syntax!"),
            MoveError::InvalidMove => write!(f, "Invalid move!"),
            MoveError::MoveIntoCheck => write!(f, "That move puts your king in danger!"),
            MoveError::AmbiguousMove =>
                write!(
                    f,
                    "Multiple pieces can make that move! Please disambiguate by providing a file, rank, or both."
                ),
            MoveError::InvalidCapture =>
                write!(f, "That move is not a capture! Please omit the 'x'."),
            MoveError::InvalidCheck =>
                write!(f, "That move is not a check! Please omit the '+' or '#'."),
            MoveError::InvalidPromotion =>
                write!(
                    f,
                    "Invalid promotion! Make sure the pawn is moving into the last rank and you specify a piece to promote into."
                ),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PieceMove {
    pub piece_type: pieces::PieceType,
    pub src_file: i8,
    pub src_rank: i8,
    pub dst_file: i8,
    pub dst_rank: i8,
}
