use crate::game;
use crate::pieces;
use crate::board;

pub struct Knight {
    data: pieces::PieceData,
}

impl Knight {
    pub fn new(data: pieces::PieceData) -> Self {
        Knight {
            data,
        }
    }
}

impl pieces::Piece for Knight {
    fn get_player(&self) -> Option<game::Player> {
        return Some(self.data.player);
    }

    fn get_type(&self) -> pieces::PieceType {
        return pieces::PieceType::Knight;
    }

    fn can_attack(&self, _: &board::Board, file: i8, rank: i8) -> bool {
        // knight can't get blocked so don't care about board state
        let diff_y = (rank - self.data.rank).abs();
        let diff_x = (file - self.data.file).abs();
        return (diff_y == 2 && diff_x == 1) || (diff_y == 1 && diff_x == 2);
    }

    fn can_move(&self, board: &board::Board, file: i8, rank: i8) -> bool {
        return self.can_attack(board, file, rank);
    }

    fn get_last_move(&self) -> Option<&pieces::PieceMove> {
        return self.data.last_move.as_ref();
    }
}

impl std::fmt::Display for Knight {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.data.player {
            game::Player::White => write!(f, "♘"),
            game::Player::Black => write!(f, "♞"),
        }
    }
}
