use std::io;

use crate::board;
use crate::notation;

#[derive(Debug, Copy, Clone)]
pub enum Player {
    White,
    Black,
}

pub enum GameState {
    Playing(Player),
    Won(Player),
    Draw,
}

pub fn other_player(p: Player) -> Player {
    match p {
        Player::White => Player::Black,
        Player::Black => Player::White,
    }
}

pub fn game_loop() {
    let mut state = GameState::Playing(Player::White);
    let mut board = board::Board::new();
    board.reset_board();

    loop {
        match state {
            GameState::Playing(p) => {
                println!("({p:?}) Enter your move:");

                let mut next_move = String::new();
                io::stdin().read_line(&mut next_move).expect("failed to read line");
                let next_move = next_move.trim();

                if next_move == "" {
                    println!("{board}");
                } else if next_move == "draw" {
                    state = GameState::Draw;
                } else if next_move == "resign" {
                    state = GameState::Won(other_player(p));
                } else {
                    let (piece, file, rank) = notation::parse_notation(&next_move);
                    //board.move_piece(p, pieces::PieceType::Pawn, 4, 1, 4, 3);
                    board.place_piece(p, piece, file, rank);
                    println!("{board}");
                    state = GameState::Playing(other_player(p));
                }
            }

            GameState::Won(p) => {
                println!("{p:?} won!");
                break;
            }

            GameState::Draw => {
                println!("Game ended in a draw!");
                break;
            }
        }
    }
}
