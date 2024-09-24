use std::io;
use crate::board;

#[derive(Debug, Copy, Clone, PartialEq)]
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

                let mut notation = String::new();
                io::stdin().read_line(&mut notation).expect("failed to read line");
                let notation = notation.trim();

                if notation == "" {
                    println!("{board}");
                } else if notation == "draw" {
                    state = GameState::Draw;
                } else if notation == "resign" {
                    state = GameState::Won(other_player(p));
                } else {
                    board.execute_move(p, notation);
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
