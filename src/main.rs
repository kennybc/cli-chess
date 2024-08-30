use std::io;

#[derive(Debug)]
enum Player {
    White,
    Black
}

enum GameState {
    Playing(Player),
    Won(Player),
    Draw
}

fn other_player(p: Player) -> Player {
    match p {
        Player::White => Player::Black,
        Player::Black => Player::White,
    }
}

fn main() {
    let mut state = GameState::Playing(Player::White);

    loop {
        match state {
            GameState::Playing(p) => {
                println!("({p:?}) Enter your move:");

                let mut next_move = String::new();
                io::stdin().read_line(&mut next_move).expect("failed to read line");
                let next_move = next_move.trim();

                if next_move == "draw" {
                    println!("draw");
                    state = GameState::Draw;
                } else if next_move == "resign" {
                    state = GameState::Won(other_player(p));
                } else {
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
