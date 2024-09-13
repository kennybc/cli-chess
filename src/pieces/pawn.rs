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

    fn can_move(&self, board: board::Board, file: u8, rank: u8) -> bool {
        let target = &board.squares[board::convert_square(file, rank)];

        match self.pos.player {
            game::Player::White => {
                if file == self.pos.file {
                    if rank == 4 && self.pos.rank == 2 {
                        return true;
                    }
                }
            }
            game::Player::Black => {}
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
