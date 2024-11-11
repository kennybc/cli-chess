use std::io;
use crate::board;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
            } else {
                let result = board.execute_notation(Some(p), &notation);
                match result {
                    Ok(_) => {
                        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                        println!("{board}");
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
