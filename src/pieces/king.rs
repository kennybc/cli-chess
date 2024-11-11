use crate::game;
use crate::pieces;
use crate::board;
use crate::moves;

#[derive(Clone)]
pub struct King {
    data: pieces::PieceData,
}

impl King {
    pub fn new(data: pieces::PieceData) -> Self {
        King {
            data,
        }
    }
}

impl pieces::Piece for King {
    fn get_player(&self) -> Option<game::Player> {
        return Some(self.data.player);
    }

    fn get_type(&self) -> pieces::PieceType {
        return pieces::PieceType::King;
    }

    fn can_attack(&self, _: &board::Board, file: i8, rank: i8) -> bool {
        let diff_y = (rank - self.data.rank).abs();
        let diff_x = (file - self.data.file).abs();
        return diff_y <= 1 && diff_x <= 1 && !(diff_x == 0 && diff_y == 0);
    }

    fn can_move(&self, board: &board::Board, file: i8, rank: i8) -> bool {
        if let Some(p) = board.squares[board::convert_position_1d(file, rank)].get_player() {
            if self.data.player == p {
                return false;
            }
        }
        let mv = moves::PieceMove::new(
            pieces::PieceType::King,
            self.data.file,
            self.data.rank,
            file,
            rank
        );
        return board.clone().piece_can_move(self.data.player, mv);
    }

    fn get_last_move(&self) -> Option<&(i32, moves::PieceMove)> {
        return self.data.last_move.as_ref();
    }

    fn set_last_move(&mut self, turn: i32, mv: moves::PieceMove) {
        self.data.last_move = Some((turn, mv));
    }
}

impl std::fmt::Display for King {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "♚")
    }
}
