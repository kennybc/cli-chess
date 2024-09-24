use crate::game;
use crate::pieces;
use crate::board;

pub struct Pawn {
    data: pieces::PieceData,
}

impl Pawn {
    pub fn new(data: pieces::PieceData) -> Self {
        Pawn {
            data,
        }
    }

    fn get_direction_coeff(&self) -> i8 {
        return match self.data.player {
            game::Player::White => 1,
            game::Player::Black => -1,
        };
    }
}

impl pieces::Piece for Pawn {
    fn get_player(&self) -> Option<game::Player> {
        return Some(self.data.player);
    }

    fn get_type(&self) -> pieces::PieceType {
        return pieces::PieceType::Pawn;
    }

    fn can_attack(&self, board: &board::Board, file: i8, mut rank: i8) -> bool {
        if file == self.data.file - 1 || file == self.data.file + 1 {
            // the rank that the pawn can reach and attack
            let reach = self.data.rank + self.get_direction_coeff();
            if rank == reach || rank == self.data.rank {
                if rank == self.data.rank {
                    rank = reach;
                    println!("{rank}");
                }
                if reach < 0 || reach > 7 {
                    return false;
                }
                if
                    board.squares[board::convert_position_1d(file, rank)].get_type() ==
                    pieces::PieceType::Empty
                {
                    // en passant
                    let en_passant_square =
                        &board.squares[board::convert_position_1d(file, self.data.rank)];
                    if en_passant_square.get_type() == pieces::PieceType::Pawn {
                        return match en_passant_square.get_last_move() {
                            None => false,
                            Some(tuple) =>
                                tuple.0 == board.get_turn() - 1 &&
                                    (tuple.1.src_rank - tuple.1.dst_rank).abs() == 2,
                        };
                    }
                } else {
                    // true for any non-empty square: pawn can attack enemy or defend ally
                    return true;
                }
            }
        }
        return false;
    }

    fn can_move(&self, board: &board::Board, file: i8, rank: i8) -> bool {
        if self.can_attack(board, file, rank) {
            return true;
        }

        let target = &board.squares[board::convert_position_1d(file, rank)];

        // pawn move within same file (non-capture move)
        if file == self.data.file {
            // target position already occupied
            if target.get_type() != pieces::PieceType::Empty {
                return false;
            }

            // allow one one square forwards
            // first move allow two squares forwards
            let diff: i8 = (rank - self.data.rank) * self.get_direction_coeff();
            return match self.get_last_move() {
                None => diff == 1 || diff == 2,
                Some(_) => diff == 1,
            };
        }

        return false;
    }

    fn get_last_move(&self) -> Option<&(i32, pieces::PieceMove)> {
        return self.data.last_move.as_ref();
    }

    fn set_last_move(&mut self, turn: i32, mv: pieces::PieceMove) {
        self.data.last_move = Some((turn, mv));
    }
}

impl std::fmt::Display for Pawn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.data.player {
            game::Player::White => write!(f, "♙"),
            game::Player::Black => write!(f, "♟"),
        }
    }
}
