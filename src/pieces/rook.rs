use crate::game;
use crate::pieces;
use crate::board;
use crate::moves;

#[derive(Clone)]
pub struct Rook {
    data: pieces::PieceData,
}

impl Rook {
    pub fn new(data: pieces::PieceData) -> Self {
        Rook {
            data,
        }
    }
}

impl pieces::Piece for Rook {
    fn get_player(&self) -> Option<game::Player> {
        return Some(self.data.player);
    }

    fn get_type(&self) -> pieces::PieceType {
        return pieces::PieceType::Rook;
    }

    fn can_attack(&self, board: &board::Board, file: i8, rank: i8) -> bool {
        if let Some(p) = board.squares[board::convert_position_1d(file, rank)].get_player() {
            if self.data.player == p {
                return false;
            }
        }
        let diff_y = (rank - self.data.rank).abs();
        let diff_x = (file - self.data.file).abs();
        if diff_y == 0 || diff_x == 0 {
            let mut curr_file = self.data.file;
            let mut curr_rank = self.data.rank;
            while (curr_file - file).abs() > 1 || (curr_rank - rank).abs() > 1 {
                if curr_file > file {
                    curr_file -= 1;
                } else if curr_file < file {
                    curr_file += 1;
                }
                if curr_rank > rank {
                    curr_rank -= 1;
                } else if curr_rank < rank {
                    curr_rank += 1;
                }
                if
                    board.squares[board::convert_position_1d(curr_file, curr_rank)].get_type() !=
                    pieces::PieceType::Empty
                {
                    return false;
                }
            }
            return true;
        }
        return false;
    }

    fn can_move(&self, board: &board::Board, file: i8, rank: i8) -> bool {
        if let Some(p) = board.squares[board::convert_position_1d(file, rank)].get_player() {
            if self.data.player == p {
                return false;
            }
        }
        let mv = moves::PieceMove {
            piece_type: pieces::PieceType::Rook,
            src_file: self.data.file,
            src_rank: self.data.rank,
            dst_file: file,
            dst_rank: rank,
        };
        return board.clone().piece_can_move(self.data.player, mv);
    }

    fn get_last_move(&self) -> Option<&(i32, moves::PieceMove)> {
        return self.data.last_move.as_ref();
    }

    fn set_last_move(&mut self, turn: i32, mv: moves::PieceMove) {
        self.data.last_move = Some((turn, mv));
    }
}

impl std::fmt::Display for Rook {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "♜")
    }
}
