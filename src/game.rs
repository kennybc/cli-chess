use crate::board::Board;
use crate::notation::parse_notation;
use crate::moves;
use std::io;

pub struct Game {
    board: Board,
    white_to_move: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            white_to_move: true,
        }
    }

    pub fn run(&mut self) {
        println!("{}", self.board);

        loop {
            println!("{} to move:", if self.white_to_move { "White" } else { "Black" });

            let mut notation = String::new();
            io::stdin().read_line(&mut notation).expect("Failed to read input");
            let notation = notation.trim();

            if notation == "quit" {
                break;
            } else if notation == "" {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                println!("{}", self.board);
            } else {
                let result = self.process_move(notation);
                match result {
                    Ok(_) => {
                        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                        println!("{}", self.board);
                    }
                    Err(e) => {
                        println!("Error: {e}");
                    }
                }
            }
        }
    }

    fn process_move(&mut self, notation: &str) -> Result<moves::Outcome, moves::Error> {
        // Step 1: Parse the notation into a move
        let parsed_move = parse_notation(&self.board, notation)?;

        // Step 2: Execute the move
        let outcome = self.board.execute_move(parsed_move)?;

        // Step 3: Return the outcome of the move
        Ok(outcome)
    }
}
