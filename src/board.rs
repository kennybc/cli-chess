use crate::pieces;
use crate::game;

#[derive(Debug)]
pub struct Board {
    squares: [Square; 64],
}

impl Board {
    pub fn new() -> Board {
        let board = Board {
            squares: [Square { piece: pieces::Piece::Empty, player: None }; 64],
        };
        return board;
    }

    pub fn reset_board(&mut self) {
        self.squares = [Square { piece: pieces::Piece::Empty, player: None }; 64];

        // white pieces
        self.place_piece(game::Player::White, pieces::Piece::Rook, 0, 0);
        self.place_piece(game::Player::White, pieces::Piece::Knight, 1, 0);
        self.place_piece(game::Player::White, pieces::Piece::Bishop, 2, 0);
        self.place_piece(game::Player::White, pieces::Piece::Queen, 3, 0);
        self.place_piece(game::Player::White, pieces::Piece::King, 4, 0);
        self.place_piece(game::Player::White, pieces::Piece::Bishop, 5, 0);
        self.place_piece(game::Player::White, pieces::Piece::Knight, 6, 0);
        self.place_piece(game::Player::White, pieces::Piece::Rook, 7, 0);
        for i in 0..8 {
            self.place_piece(game::Player::White, pieces::Piece::Pawn, i, 1);
        }

        // black pieces
        self.place_piece(game::Player::Black, pieces::Piece::Rook, 0, 7);
        self.place_piece(game::Player::Black, pieces::Piece::Knight, 1, 7);
        self.place_piece(game::Player::Black, pieces::Piece::Bishop, 2, 7);
        self.place_piece(game::Player::Black, pieces::Piece::Queen, 3, 7);
        self.place_piece(game::Player::Black, pieces::Piece::King, 4, 7);
        self.place_piece(game::Player::Black, pieces::Piece::Bishop, 5, 7);
        self.place_piece(game::Player::Black, pieces::Piece::Knight, 6, 7);
        self.place_piece(game::Player::Black, pieces::Piece::Rook, 7, 7);
        for i in 0..8 {
            self.place_piece(game::Player::Black, pieces::Piece::Pawn, i, 6);
        }
    }

    pub fn place_piece(&mut self, player: game::Player, piece: pieces::Piece, file: u8, rank: u8) {
        let pos = convert_square(file, rank);
        self.squares[pos].piece = piece;
        self.squares[pos].player = Some(player);
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board_string = String::new();
        for (i, square) in self.squares.iter().enumerate() {
            board_string += &square.to_string();
            board_string += " ";
            let col = (i as u8) % 8;
            if col == 7 {
                board_string += "\n";
            }
        }
        return write!(f, "{board_string}");
    }
}

// convert a file and rank to a square index on a 1d board array
fn convert_square(file: u8, rank: u8) -> usize {
    return (8 * (7 - rank) + file) as usize;
}

#[derive(Debug, Copy, Clone)]
pub struct Square {
    pub piece: pieces::Piece,
    pub player: Option<game::Player>,
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let piece = match self.player {
            None => "_",
            Some(player) => {
                match player {
                    game::Player::White =>
                        match self.piece {
                            pieces::Piece::King => "♔",
                            pieces::Piece::Queen => "♕",
                            pieces::Piece::Rook => "♖",
                            pieces::Piece::Bishop => "♗",
                            pieces::Piece::Knight => "♘",
                            pieces::Piece::Pawn => "♙",
                            pieces::Piece::Empty => "_",
                        }
                    game::Player::Black =>
                        match self.piece {
                            pieces::Piece::King => "♚",
                            pieces::Piece::Queen => "♛",
                            pieces::Piece::Rook => "♜",
                            pieces::Piece::Bishop => "♝",
                            pieces::Piece::Knight => "♞",
                            pieces::Piece::Pawn => "♟",
                            pieces::Piece::Empty => "_",
                        }
                }
            }
        };
        return write!(f, "{piece}");
    }
}
