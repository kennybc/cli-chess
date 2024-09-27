use colored::Colorize;

use crate::moves::PieceMove;
use crate::notation;
use crate::pieces;
use crate::game;
use crate::moves;

pub struct Board {
    pub turn: i32,
    pub state: game::GameState,
    pub squares: [Box<dyn pieces::Piece>; 64],
    pub white_king: (i8, i8),
    pub black_king: (i8, i8),
}

impl Board {
    pub fn new() -> Board {
        let board = Board {
            turn: 0,
            state: game::GameState::Playing(game::Player::White),
            squares: array_init::array_init(
                |_| Box::new(pieces::empty::Empty {}) as Box<dyn pieces::Piece>
            ),
            white_king: (0, 0),
            black_king: (0, 0),
        };
        return board;
    }

    pub fn get_turn(&self) -> i32 {
        return self.turn;
    }

    pub fn get_state(&self) -> &game::GameState {
        return &self.state;
    }

    pub fn set_state(&mut self, state: game::GameState) {
        self.state = state;
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

        self.white_king = (4, 0);
        self.black_king = (4, 7);
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

    pub fn execute_move(
        &mut self,
        player: game::Player,
        notation: &str
    ) -> Result<moves::MoveOutcome, moves::MoveError> {
        let notation = notation.to_ascii_lowercase();

        // handle "special" castling notation first
        if notation == "o-o" || notation == "o-o-o" {
            let castle_rank = match player {
                game::Player::White => 0,
                game::Player::Black => 7,
            };
            let mut king_file: Option<i8> = None;
            let mut rook_src_file: Option<i8> = None;
            let mut rook_dst_file: Option<i8> = None;
            // king side castle
            if notation == "o-o" && self.can_king_castle(castle_rank) {
                self.place_piece(player, pieces::PieceType::King, 6, castle_rank);
                self.place_piece(player, pieces::PieceType::Rook, 5, castle_rank);
                self.clear_square(4, castle_rank);
                self.clear_square(7, castle_rank);
                king_file = Some(6);
                rook_src_file = Some(7);
                rook_dst_file = Some(5);
            } else if notation == "o-o-o" && self.can_queen_castle(castle_rank) {
                self.place_piece(player, pieces::PieceType::King, 2, castle_rank);
                self.place_piece(player, pieces::PieceType::Rook, 3, castle_rank);
                self.clear_square(4, castle_rank);
                self.clear_square(0, castle_rank);
                king_file = Some(2);
                rook_src_file = Some(0);
                rook_dst_file = Some(3);
            } else {
                return Err(moves::MoveError::InvalidMove);
            }

            let king_file = king_file.unwrap();
            let rook_src_file = rook_src_file.unwrap();
            let rook_dst_file = rook_dst_file.unwrap();

            self.squares[convert_position_1d(king_file, castle_rank)].set_last_move(
                self.turn,
                PieceMove {
                    piece_type: pieces::PieceType::King,
                    src_file: 4,
                    src_rank: castle_rank,
                    dst_file: king_file,
                    dst_rank: castle_rank,
                }
            );
            self.squares[convert_position_1d(rook_dst_file, castle_rank)].set_last_move(
                self.turn,
                PieceMove {
                    piece_type: pieces::PieceType::King,
                    src_file: rook_src_file,
                    src_rank: castle_rank,
                    dst_file: rook_dst_file,
                    dst_rank: castle_rank,
                }
            );
            self.turn += 1;
            return Ok(moves::MoveOutcome::Continue);
        } else {
            let piece_move = notation::parse_notation(self, &player, &notation);
            match piece_move {
                Ok(mut mv) => {
                    let mut removed_pieces: Vec<(i8, i8, Box<dyn pieces::Piece>)> = Vec::new();

                    let src_index = convert_position_1d(mv.src_file, mv.src_rank);
                    let dst_index = convert_position_1d(mv.dst_file, mv.dst_rank);

                    let is_king_move =
                        self.squares[src_index].get_type() == pieces::PieceType::King;
                    // update stored king positions if it is a king move
                    if is_king_move {
                        match player {
                            game::Player::White => {
                                self.white_king = (mv.dst_file, mv.dst_rank);
                            }
                            game::Player::Black => {
                                self.black_king = (mv.dst_file, mv.dst_rank);
                            }
                        }
                    }

                    // check if it is en passant
                    if
                        self.squares[src_index].get_type() == pieces::PieceType::Pawn &&
                        mv.src_rank == mv.dst_rank
                    {
                        let direction_coef = match player {
                            game::Player::White => 1,
                            game::Player::Black => -1,
                        };

                        // temporarily save the removed pawn
                        removed_pieces.push((
                            mv.dst_file,
                            mv.src_rank,
                            dyn_clone::clone_box(
                                &*self.squares[convert_position_1d(mv.dst_file, mv.src_rank)]
                            ),
                        ));

                        self.clear_square(mv.dst_file, mv.src_rank);
                        mv.dst_rank += direction_coef;
                    }
                    // temporary save the moved pieces
                    removed_pieces.push((
                        mv.dst_file,
                        mv.dst_rank,
                        dyn_clone::clone_box(
                            &*self.squares[convert_position_1d(mv.dst_file, mv.dst_rank)]
                        ),
                    ));
                    removed_pieces.push((
                        mv.src_file,
                        mv.src_rank,
                        dyn_clone::clone_box(
                            &*self.squares[convert_position_1d(mv.src_file, mv.src_rank)]
                        ),
                    ));
                    self.clear_square(mv.src_file, mv.src_rank);
                    self.place_piece(player, mv.piece_type, mv.dst_file, mv.dst_rank);

                    let (enemy_checking_pieces, ally_checking_pieces) =
                        self.get_checking_pieces(player);

                    // if move results in being checked, undo the move and throw error
                    if enemy_checking_pieces.len() > 0 {
                        // undo moves
                        for removed_piece in removed_pieces {
                            self.squares[convert_position_1d(removed_piece.0, removed_piece.1)] =
                                removed_piece.2;
                        }

                        if is_king_move {
                            match player {
                                game::Player::White => {
                                    self.white_king = (mv.src_file, mv.src_rank);
                                }
                                game::Player::Black => {
                                    self.black_king = (mv.src_file, mv.src_rank);
                                }
                            }
                        }
                        return Err(moves::MoveError::MoveIntoCheck);
                    }

                    self.squares[dst_index].set_last_move(self.turn, mv);
                    self.turn += 1;

                    return Ok(moves::MoveOutcome::Continue);
                }
                Err(e) => Err(e),
            }
        }
    }

    // returns a tuple (0, 1) where:
    // 0: a list of enemy pieces attacking friendly king
    // 1: a list of friendly pieces attacking enemy king
    fn get_checking_pieces(&self, defender: game::Player) -> (Vec<(i8, i8)>, Vec<(i8, i8)>) {
        let mut enemies: Vec<(i8, i8)> = Vec::new();
        let mut allies: Vec<(i8, i8)> = Vec::new();

        let (king, enemy_king) = match defender {
            game::Player::White => (self.white_king, self.black_king),
            game::Player::Black => (self.black_king, self.white_king),
        };

        for f in 0..8 {
            for r in 0..8 {
                let attacker = &self.squares[convert_position_1d(f, r)];
                if let Some(p) = attacker.get_player() {
                    // if enemy piece, check if can attack given square
                    if p != defender {
                        if attacker.can_attack(self, king.0, king.1) {
                            enemies.push((f, r));
                        }
                    } else {
                        // if friendly piece, check if can attack enemy king
                        if attacker.can_attack(self, enemy_king.0, enemy_king.1) {
                            allies.push((f, r));
                        }
                    }
                }
            }
        }
        return (enemies, allies);
    }

    // check if a player can castle (helper)
    fn can_castle(&mut self, castle_rank: i8, rook_file: i8) -> bool {
        if castle_rank != 0 && castle_rank != 7 {
            return false;
        }
        let king = &self.squares[convert_position_1d(4, castle_rank)];
        let rook = &self.squares[convert_position_1d(rook_file, castle_rank)];
        let defender = if castle_rank == 0 { game::Player::White } else { game::Player::Black };

        // check if king and rook are in place
        if
            king.get_type() == pieces::PieceType::King &&
            rook.get_type() == pieces::PieceType::Rook &&
            king.get_last_move() == None &&
            rook.get_last_move() == None
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
                for j in 0..64 {
                    let attacker = &self.squares[j];
                    if let Some(p) = attacker.get_player() {
                        // if enemy piece, check if can attack given square
                        if p != defender && attacker.can_attack(self, i, castle_rank) {
                            return false;
                        }
                    }
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
            let mut piece_string = piece.to_string() + " ";
            let row = i / 8;
            let col = i % 8;
            if (row + col) % 2 == 0 {
                piece_string = piece_string.on_truecolor(240, 240, 240).to_string();
            } else {
                piece_string = piece_string.on_truecolor(202, 202, 202).to_string();
            }
            if let Some(p) = piece.get_player() {
                if p == game::Player::Black {
                    piece_string = piece_string.blue().to_string();
                } else {
                    piece_string = piece_string.red().to_string();
                }
            }
            board_string += &piece_string;
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
