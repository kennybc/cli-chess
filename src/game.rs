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
    let mut board = board::Board::new();
    board.reset_board();

    loop {
        if let &GameState::Playing(p) = board.get_state() {
            println!("({p:?}) Enter your move:");
            let notation = String::new();
            let mut notation = String::new();
            io::stdin().read_line(&mut notation).expect("failed to read line");
            let notation = notation.trim();

            if notation == "" {
                println!("{board}");
            } else if notation == "draw" {
                board.set_state(GameState::Draw);
            } else if notation == "resign" {
                board.set_state(GameState::Won(other_player(p)));
            } else {
                board.execute_move(p, &notation);
                println!("{board}");
                board.set_state(GameState::Playing(other_player(p)));
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
