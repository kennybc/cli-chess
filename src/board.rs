use crate::pieces;
use crate::game;

pub struct Board {
    pub squares: [Box<dyn pieces::Piece>; 64],
}

impl Board {
    pub fn new() -> Board {
        let board = Board {
            squares: array_init::array_init(
                |_| Box::new(pieces::empty::Empty {}) as Box<dyn pieces::Piece>
            ),
        };
        return board;
    }

    pub fn reset_board(&mut self) {
        for i in 0..64 {
            self.squares[i] = Box::new(pieces::empty::Empty {}) as Box<dyn pieces::Piece>;
        }

        // white pieces
        self.place_piece(game::Player::White, pieces::PieceType::Rook, 0, 0);
        self.place_piece(game::Player::White, pieces::PieceType::Knight, 1, 0);
        self.place_piece(game::Player::White, pieces::PieceType::Bishop, 2, 0);
        self.place_piece(game::Player::White, pieces::PieceType::Queen, 3, 0);
        self.place_piece(game::Player::White, pieces::PieceType::King, 4, 0);
        self.place_piece(game::Player::White, pieces::PieceType::Bishop, 5, 0);
        self.place_piece(game::Player::White, pieces::PieceType::Knight, 6, 0);
        self.place_piece(game::Player::White, pieces::PieceType::Rook, 7, 0);
        for i in 0..8 {
            self.place_piece(game::Player::White, pieces::PieceType::Pawn, i, 1);
        }

        // black pieces
        self.place_piece(game::Player::Black, pieces::PieceType::Rook, 0, 7);
        self.place_piece(game::Player::Black, pieces::PieceType::Knight, 1, 7);
        self.place_piece(game::Player::Black, pieces::PieceType::Bishop, 2, 7);
        self.place_piece(game::Player::Black, pieces::PieceType::Queen, 3, 7);
        self.place_piece(game::Player::Black, pieces::PieceType::King, 4, 7);
        self.place_piece(game::Player::Black, pieces::PieceType::Bishop, 5, 7);
        self.place_piece(game::Player::Black, pieces::PieceType::Knight, 6, 7);
        self.place_piece(game::Player::Black, pieces::PieceType::Rook, 7, 7);
        for i in 0..8 {
            self.place_piece(game::Player::Black, pieces::PieceType::Pawn, i, 6);
        }
    }

    // place a piece regardless of move validity
    // does not update pieces' move history
    // usage: initializing/resetting the board
    pub fn place_piece(
        &mut self,
        player: game::Player,
        piece_type: pieces::PieceType,
        file: u8,
        rank: u8
    ) {
        let pos = pieces::PiecePosition { player, file, rank };
        let index = convert_position_1d(file, rank);
        self.squares[index] = new_boxed_piece(piece_type, pos);
    }

    pub fn move_piece(
        &mut self,
        player: game::Player,
        piece_type: pieces::PieceType,
        piece_move: pieces::PieceMove
    ) {
        let pos = pieces::PiecePosition {
            player,
            file: piece_move.dst_file,
            rank: piece_move.dst_rank,
        };
        let src_index = convert_position_1d(piece_move.src_file, piece_move.src_rank);
        let dst_index = convert_position_1d(piece_move.dst_file, piece_move.dst_rank);
        if self.squares[src_index].can_move(self, pos) {
            self.squares[src_index] = new_boxed_piece(pieces::PieceType::Empty, pos);
            self.squares[dst_index] = new_boxed_piece(piece_type, pos);
        } else {
            println!("invalid move!")
        }
    }

    fn is_valid_move(
        &mut self,
        player: game::Player,
        piece: pieces::PieceType,
        file: u8,
        rank: u8
    ) {
        // get possible "source" squares given the player + piece
        // check if any instances of given player + piece exist on board
        // if multiple instances, require disambiguating source position
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board_string = String::new();
        for (i, piece) in self.squares.iter().enumerate() {
            board_string += &piece.to_string();
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
pub fn convert_position_1d(file: u8, rank: u8) -> usize {
    return (8 * (7 - rank) + file) as usize;
}

pub fn new_boxed_piece(
    piece_type: pieces::PieceType,
    pos: pieces::PiecePosition
) -> Box<dyn pieces::Piece> {
    match piece_type {
        pieces::PieceType::King =>
            Box::new(pieces::king::King { pos, last_move: None }) as Box<dyn pieces::Piece>,
        pieces::PieceType::Queen =>
            Box::new(pieces::queen::Queen { pos, last_move: None }) as Box<dyn pieces::Piece>,
        pieces::PieceType::Rook =>
            Box::new(pieces::rook::Rook { pos, last_move: None }) as Box<dyn pieces::Piece>,
        pieces::PieceType::Bishop =>
            Box::new(pieces::bishop::Bishop { pos, last_move: None }) as Box<dyn pieces::Piece>,
        pieces::PieceType::Knight =>
            Box::new(pieces::knight::Knight { pos, last_move: None }) as Box<dyn pieces::Piece>,
        pieces::PieceType::Pawn =>
            Box::new(pieces::pawn::Pawn { pos, last_move: None }) as Box<dyn pieces::Piece>,
        pieces::PieceType::Empty => Box::new(pieces::empty::Empty {}) as Box<dyn pieces::Piece>,
    }
}
