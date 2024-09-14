use crate::pieces;
use crate::game;
use crate::pieces::Position;

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
    pub fn place_piece(
        &mut self,
        player: game::Player,
        piece_type: pieces::PieceType,
        file: u8,
        rank: u8
    ) {
        let pos = pieces::Position { player, file, rank };
        let index = convert_square(file, rank);
        self.squares[index] = new_boxed_piece(piece_type, pos);
    }

    pub fn move_piece(
        &mut self,
        player: game::Player,
        piece_type: pieces::PieceType,
        src_file: u8,
        src_rank: u8,
        dst_file: u8,
        dst_rank: u8
    ) {
        let pos = pieces::Position { player, file: dst_file, rank: dst_rank };
        let src_index = convert_square(src_file, src_rank);
        let dst_index = convert_square(dst_file, dst_rank);
        if self.squares[src_index].can_move(self, dst_file, dst_rank) {
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
pub fn convert_square(file: u8, rank: u8) -> usize {
    return (8 * (7 - rank) + file) as usize;
}

pub fn new_boxed_piece(
    piece_type: pieces::PieceType,
    pos: pieces::Position
) -> Box<dyn pieces::Piece> {
    match piece_type {
        pieces::PieceType::King => Box::new(pieces::king::King { pos }) as Box<dyn pieces::Piece>,
        pieces::PieceType::Queen =>
            Box::new(pieces::queen::Queen { pos }) as Box<dyn pieces::Piece>,
        pieces::PieceType::Rook => Box::new(pieces::rook::Rook { pos }) as Box<dyn pieces::Piece>,
        pieces::PieceType::Bishop =>
            Box::new(pieces::bishop::Bishop { pos }) as Box<dyn pieces::Piece>,
        pieces::PieceType::Knight =>
            Box::new(pieces::knight::Knight { pos }) as Box<dyn pieces::Piece>,
        pieces::PieceType::Pawn => Box::new(pieces::pawn::Pawn { pos }) as Box<dyn pieces::Piece>,
        pieces::PieceType::Empty => Box::new(pieces::empty::Empty {}) as Box<dyn pieces::Piece>,
    }
}
