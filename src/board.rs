use std::fmt::Result;

use colored::Colorize;

use crate::moves;
use crate::pieces;

pub struct Board {
    bitboards: [[u64; 6]; 2], // [player][piece]
}

impl Board {
    pub fn new() -> Self {
        Self {
            bitboards: [
                [
                    // white pieces
                    0x000000000000ff00, // pawns
                    0x0000000000000042, // knights
                    0x0000000000000024, // bishops
                    0x0000000000000081, // rooks
                    0x0000000000000008, // queen
                    0x0000000000000010, // king
                ],
                [
                    // black pieces
                    0x00ff000000000000, // pawns
                    0x4200000000000000, // knights
                    0x2400000000000000, // bishops
                    0x8100000000000000, // rooks
                    0x0800000000000000, // queen
                    0x1000000000000000, // king
                ],
            ],
        }
    }

    pub fn execute_move(
        &self,
        _move: moves::Move
    ) -> core::result::Result<moves::Outcome, moves::Error> {
        return Err(moves::Error::InvalidNotation);
    }

    fn get_bitboard(&self, player: pieces::Player, piece: pieces::Piece) -> u64 {
        self.bitboards[player as usize][piece as usize]
    }

    fn set_bitboard(&mut self, player: pieces::Player, piece: pieces::Piece, value: u64) {
        self.bitboards[player as usize][piece as usize] = value;
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board_string = String::new();

        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                let mut piece_char = ' ';

                for (player_idx, player_bitboards) in self.bitboards.iter().enumerate() {
                    for (piece_idx, &bitboard) in player_bitboards.iter().enumerate() {
                        if (bitboard & (1 << square)) != 0 {
                            if player_idx == 0 {
                                piece_char = match piece_idx {
                                    0 => '♙',
                                    1 => '♘',
                                    2 => '♗',
                                    3 => '♖',
                                    4 => '♕',
                                    5 => '♔',
                                    _ => piece_char,
                                };
                            } else {
                                piece_char = match piece_idx {
                                    0 => '♟',
                                    1 => '♞',
                                    2 => '♝',
                                    3 => '♜',
                                    4 => '♛',
                                    5 => '♚',
                                    _ => piece_char,
                                };
                            }
                        }
                    }
                }

                let mut piece_string = piece_char.to_string() + " ";

                // color board squares
                if (rank + file) % 2 == 0 {
                    piece_string = piece_string.on_truecolor(240, 240, 240).to_string();
                } else {
                    piece_string = piece_string.on_truecolor(202, 202, 202).to_string();
                }

                // color pieces
                /*if piece_char.is_uppercase() {
                    piece_string = piece_string.red().to_string();
                } else if piece_char.is_lowercase() {
                    piece_string = piece_string.blue().to_string();
                }*/

                board_string += &piece_string;
            }
            board_string += "\n";
        }

        write!(f, "{board_string}")
    }
}

// convert a file and rank to a square index on a 1d board array
pub fn convert_position_1d(file: i8, rank: i8) -> usize {
    return (8 * (7 - rank) + file) as usize;
}
