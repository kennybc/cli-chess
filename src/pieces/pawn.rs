use crate::game;
use crate::pieces;
use crate::board;

pub struct Pawn {
    pub pos: pieces::PiecePosition,
    pub last_move: Option<pieces::PieceMove>,
}

impl pieces::Piece for Pawn {
    fn get_type(&self) -> pieces::PieceType {
        return pieces::PieceType::Pawn;
    }

    fn can_move(&self, board: &board::Board, pos: pieces::PiecePosition) -> bool {
        let target = &board.squares[board::convert_position_1d(pos.file, pos.rank)];

        // pawn move within same file (non-capture move)
        if pos.file == self.pos.file {
            // target position already occupied
            if let pieces::PieceType::Empty = target.get_type() {
            } else {
                return false;
            }

            // allow one one square forwards
            // first move allow two squares forwards
            let multiplier: i8 = match self.pos.player {
                game::Player::White => 1,
                game::Player::Black => -1,
            };
            let diff: i8 = ((pos.rank as i8) - (self.pos.rank as i8)) * multiplier;
            return match self.last_move {
                None => diff == 1 || diff == 2,
                Some(_) => diff == 1,
            };
        }

        return false;
    }

    fn get_last_move(&self) -> Option<&pieces::PieceMove> {
        return self.last_move.as_ref();
    }
}

impl std::fmt::Display for Pawn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.pos.player {
            game::Player::White => write!(f, "♙"),
            game::Player::Black => write!(f, "♟"),
        }
    }
}
