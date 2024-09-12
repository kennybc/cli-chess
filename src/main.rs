use std::io;

mod game;
mod pieces;
mod board;
mod notation;

fn main() {
    let mut state = game::GameState::Playing(game::Player::White);
    let mut board = board::Board::new();
    board.reset_board();

    loop {
        match state {
            game::GameState::Playing(p) => {
                println!("({p:?}) Enter your move:");

                let mut next_move = String::new();
                io::stdin().read_line(&mut next_move).expect("failed to read line");
                let next_move = next_move.trim();

                if next_move == "" {
                    println!("{board}");
                } else if next_move == "draw" {
                    state = game::GameState::Draw;
                } else if next_move == "resign" {
                    state = game::GameState::Won(game::other_player(p));
                } else {
                    let (piece, file, rank) = notation::parse_notation(&next_move);
                    board.place_piece(p, piece, file, rank);
                    println!("{board}");
                    state = game::GameState::Playing(game::other_player(p));
                }
            }

            game::GameState::Won(p) => {
                println!("{p:?} won!");
                break;
            }

            game::GameState::Draw => {
                println!("Game ended in a draw!");
                break;
            }
        }
    }
}
