use std::io;
use crate::{ board, moves };

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
    let mut board = board::Board::new();
    board.reset_board();
    println!("{board}");

    loop {
        if let &GameState::Playing(p) = board.get_state() {
            println!("({p:?}) Enter your move:");
            let mut notation = String::new();
            io::stdin().read_line(&mut notation).expect("failed to read line");
            let notation = notation.trim();

            if notation == "" {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                println!("{board}");
            } else if notation == "draw" {
                board.set_state(GameState::Draw);
            } else if notation == "resign" {
                board.set_state(GameState::Won(other_player(p)));
            } else {
                let result = board.execute_move(p, &notation);
                match result {
                    Ok(outcome) => {
                        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                        println!("{board}");
                        match outcome {
                            moves::MoveOutcome::Continue => {
                                board.set_state(GameState::Playing(other_player(p)));
                            }
                            moves::MoveOutcome::Checkmate => {
                                board.set_state(GameState::Won(p));
                            }
                            moves::MoveOutcome::Draw => {
                                board.set_state(GameState::Draw);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error: {e}");
                    }
                }
            }
        } else {
            break;
        }
    }

    if let GameState::Won(p) = board.get_state() {
        println!("{p:?} won!");
    }

    if let GameState::Draw = board.get_state() {
        println!("Game ended in a draw!");
    }
}
