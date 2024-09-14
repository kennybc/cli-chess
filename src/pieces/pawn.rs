use crate::game;
use crate::pieces;
use crate::board;

pub struct Pawn {
    pub pos: pieces::Position,
}

impl pieces::Piece for Pawn {
    fn get_type(&self) -> pieces::PieceType {
        return pieces::PieceType::Pawn;
    }

    fn can_move(&self, board: &board::Board, file: u8, rank: u8) -> bool {
        let target = &board.squares[board::convert_square(file, rank)];

        // pawn move within same file (non-capture move)
        if file == self.pos.file {
            // target position already occupied
            if let pieces::PieceType::Empty = target.get_type() {
            } else {
                return false;
            }

            // allow one one square forwards
            // first move allow two squares forwards
            let diff: i8 = (rank as i8) - (self.pos.rank as i8);
            let other = self.pos.rank;
            println!("src: {other}");
            println!("dst: {rank}");
            match self.pos.player {
                game::Player::White => {
                    return diff == 1 || (rank == 3 && self.pos.rank == 1);
                }
                game::Player::Black => {
                    return diff == -1 || (rank == 5 && self.pos.rank == 7);
                }
            }
        }

        return false;
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
