#![allow(unused_variables)]

use crate::game;
use crate::pieces;
use crate::board;

pub struct Queen {
    data: pieces::PieceData,
}

impl Queen {
    pub fn new(data: pieces::PieceData) -> Self {
        Queen {
            data,
        }
    }
}

impl pieces::Piece for Queen {
    fn get_player(&self) -> Option<game::Player> {
        return Some(self.data.player);
    }

    fn get_type(&self) -> pieces::PieceType {
        return pieces::PieceType::Queen;
    }

    fn can_attack(&self, board: &board::Board, file: i8, rank: i8) -> bool {
        return false;
    }

    fn can_move(&self, board: &board::Board, file: i8, rank: i8) -> bool {
        return false;
    }

    fn get_last_move(&self) -> Option<&pieces::PieceMove> {
        return self.data.last_move.as_ref();
    }
}

impl std::fmt::Display for Queen {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.data.player {
            game::Player::White => write!(f, "♕"),
            game::Player::Black => write!(f, "♛"),
        }
    }
}
