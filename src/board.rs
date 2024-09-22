use crate::notation;
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

        /*// white pieces
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
        }*/

        self.place_piece(game::Player::Black, pieces::PieceType::Knight, 0, 0);
        self.place_piece(game::Player::Black, pieces::PieceType::Knight, 0, 2);
        self.place_piece(game::Player::Black, pieces::PieceType::Knight, 4, 0);
        self.place_piece(game::Player::White, pieces::PieceType::Knight, 4, 2);
    }

    // place a piece regardless of move validity
    // does not update pieces' move history
    // usage: initializing/resetting the board
    pub fn place_piece(
        &mut self,
        player: game::Player,
        piece_type: pieces::PieceType,
        file: i8,
        rank: i8
    ) {
        let index = convert_position_1d(file, rank);
        self.squares[index] = new_boxed_piece(player, piece_type, file, rank);
    }

    // clear a square
    pub fn clear_square(&mut self, file: i8, rank: i8) {
        let index = convert_position_1d(file, rank);
        self.squares[index] = Box::new(pieces::empty::Empty {}) as Box<dyn pieces::Piece>;
    }

    pub fn execute_move(&mut self, player: game::Player, notation: &str) {
        // handle "special" castling notation first
        if notation == "O-O" || notation == "O-O-O" {
            let castle_rank = match player {
                game::Player::White => 0,
                game::Player::Black => 7,
            };
            // king side castle
            if notation == "O-O" && self.can_king_castle(castle_rank) {
                self.place_piece(player, pieces::PieceType::King, 6, castle_rank);
                self.place_piece(player, pieces::PieceType::Rook, 5, castle_rank);
                self.clear_square(4, castle_rank);
                self.clear_square(7, castle_rank);
            }
            // queen side castle
            if notation == "O-O-O" && self.can_queen_castle(castle_rank) {
                self.place_piece(player, pieces::PieceType::King, 2, castle_rank);
                self.place_piece(player, pieces::PieceType::Rook, 3, castle_rank);
                self.clear_square(4, castle_rank);
                self.clear_square(0, castle_rank);
            }
        } else {
            let piece_move = notation
                ::parse_notation(self, &player, notation)
                .expect("invalid move");
            let src_index = convert_position_1d(piece_move.src_file, piece_move.src_rank);
            let dst_index = convert_position_1d(piece_move.dst_file, piece_move.dst_rank);
            if self.squares[src_index].can_move(self, piece_move.dst_file, piece_move.dst_rank) {
                self.clear_square(piece_move.src_file, piece_move.src_rank);
                self.squares[dst_index] = new_boxed_piece(
                    player,
                    piece_move.piece_type,
                    piece_move.dst_file,
                    piece_move.dst_rank
                );
            } else {
                println!("invalid move!")
            }
        }
    }

    /*fn is_valid_move(
        &mut self,
        player: game::Player,
        piece: pieces::PieceType,
        file: i8,
        rank: i8
    ) {
        // get possible "source" squares given the player + piece
        // check if any instances of given player + piece exist on board
        // if multiple instances, require disambiguating source position
    }*/

    // is a square under attack?
    pub fn is_under_attack(&self, file: i8, rank: i8) -> bool {
        return false;
    }

    // check if a player can castle (helper)
    fn can_castle(&mut self, castle_rank: i8, rook_file: i8) -> bool {
        let king_index = convert_position_1d(4, castle_rank);
        let rook_index = convert_position_1d(rook_file, castle_rank);

        // check if king and rook are in place
        if
            self.squares[king_index].get_type() == pieces::PieceType::King &&
            self.squares[rook_index].get_type() == pieces::PieceType::Rook
        {
            // ensure squares between king and rook are empty
            let castle_path = if rook_file == 0 { 1..4 } else { 5..7 };
            for i in castle_path {
                if
                    self.squares[convert_position_1d(i, castle_rank)].get_type() !=
                    pieces::PieceType::Empty
                {
                    return false;
                }
            }

            // ensure that king will not pass through check during castle
            let king_path = if rook_file == 0 { 2..5 } else { 4..7 };
            for i in king_path {
                if self.is_under_attack(i, castle_rank) {
                    return false;
                }

                return true;
            }
        }
        return false;
    }

    // can a player castle king side?
    fn can_king_castle(&mut self, castle_rank: i8) -> bool {
        return self.can_castle(castle_rank, 7);
    }

    // can a player castle queen side?
    fn can_queen_castle(&mut self, castle_rank: i8) -> bool {
        return self.can_castle(castle_rank, 0);
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board_string = String::new();
        for (i, piece) in self.squares.iter().enumerate() {
            board_string += &piece.to_string();
            board_string += " ";
            let col = i % 8;
            if col == 7 {
                board_string += "\n";
            }
        }
        return write!(f, "{board_string}");
    }
}

// convert a file and rank to a square index on a 1d board array
pub fn convert_position_1d(file: i8, rank: i8) -> usize {
    return (8 * (7 - rank) + file) as usize;
}

pub fn new_boxed_piece(
    player: game::Player,
    piece_type: pieces::PieceType,
    file: i8,
    rank: i8
) -> Box<dyn pieces::Piece> {
    if let pieces::PieceType::Empty = piece_type {
        return Box::new(pieces::empty::Empty {}) as Box<dyn pieces::Piece>;
    } else {
        let piece_data: pieces::PieceData = pieces::PieceData {
            player,
            file,
            rank,
            last_move: None,
        };
        match piece_type {
            pieces::PieceType::King =>
                Box::new(pieces::king::King::new(piece_data)) as Box<dyn pieces::Piece>,
            pieces::PieceType::Queen =>
                Box::new(pieces::queen::Queen::new(piece_data)) as Box<dyn pieces::Piece>,
            pieces::PieceType::Rook =>
                Box::new(pieces::rook::Rook::new(piece_data)) as Box<dyn pieces::Piece>,
            pieces::PieceType::Bishop =>
                Box::new(pieces::bishop::Bishop::new(piece_data)) as Box<dyn pieces::Piece>,
            pieces::PieceType::Knight =>
                Box::new(pieces::knight::Knight::new(piece_data)) as Box<dyn pieces::Piece>,
            pieces::PieceType::Pawn =>
                Box::new(pieces::pawn::Pawn::new(piece_data)) as Box<dyn pieces::Piece>,
            pieces::PieceType::Empty => Box::new(pieces::empty::Empty {}) as Box<dyn pieces::Piece>,
        }
    }
}
