use colored::Colorize;

use crate::game::other_player;
use crate::game::GameState;
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
        Board {
            turn: 0,
            state: game::GameState::Playing(game::Player::White),
            squares: array_init::array_init(
                |_| Box::new(pieces::empty::Empty {}) as Box<dyn pieces::Piece>
            ),
            white_king: (0, 0),
            black_king: (0, 0),
        }
    }

    pub fn clone(&self) -> Board {
        let mut clone = Board {
            turn: self.turn,
            state: self.state,
            squares: array_init::array_init(
                |_| Box::new(pieces::empty::Empty {}) as Box<dyn pieces::Piece>
            ),
            white_king: (self.white_king.0, self.white_king.1),
            black_king: (self.black_king.0, self.black_king.1),
        };
        for f in 0..8 {
            for r in 0..8 {
                let piece = &self.squares[convert_position_1d(f, r)];
                if let Some(p) = piece.get_player() {
                    clone.squares[convert_position_1d(f, r)] = new_boxed_piece(
                        p,
                        piece.get_type(),
                        f,
                        r
                    );
                }
            }
        }
        return clone;
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

    pub fn piece_can_move(&self, player: game::Player, mv: moves::PieceMove) -> bool {
        let piece = &self.squares[convert_position_1d(mv.src_file, mv.src_rank)];
        if piece.can_attack(self, mv.dst_file, mv.dst_rank) {
            return match self.clone().execute_move(Some(player), mv) {
                Ok(_) => true,
                Err(_) => false,
            };
        }
        return false;
    }

    fn execute_move(
        &mut self,
        player: Option<game::Player>,
        mut mv: moves::PieceMove
    ) -> Result<moves::MoveOutcome, moves::MoveError> {
        // check if game still in playing state; extract current player
        let player = player.unwrap_or(match self.state {
            game::GameState::Playing(p) => p,
            game::GameState::Draw => {
                return Err(moves::MoveError::InvalidMove);
            }
            game::GameState::Won(_) => {
                return Err(moves::MoveError::InvalidMove);
            }
        });

        let mut removed_pieces: Vec<(i8, i8, Box<dyn pieces::Piece>)> = Vec::new();

        let src_index = convert_position_1d(mv.src_file, mv.src_rank);
        let dst_index = convert_position_1d(mv.dst_file, mv.dst_rank);

        let is_king_move = self.squares[src_index].get_type() == pieces::PieceType::King;
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
                dyn_clone::clone_box(&*self.squares[convert_position_1d(mv.dst_file, mv.src_rank)]),
            ));

            self.clear_square(mv.dst_file, mv.src_rank);
            mv.dst_rank += direction_coef;
        }

        // temporary save the moved pieces
        removed_pieces.push((
            mv.dst_file,
            mv.dst_rank,
            dyn_clone::clone_box(&*self.squares[convert_position_1d(mv.dst_file, mv.dst_rank)]),
        ));
        removed_pieces.push((
            mv.src_file,
            mv.src_rank,
            dyn_clone::clone_box(&*self.squares[convert_position_1d(mv.src_file, mv.src_rank)]),
        ));
        self.clear_square(mv.src_file, mv.src_rank);
        self.place_piece(player, mv.piece_type, mv.dst_file, mv.dst_rank);

        let (enemy_checking_pieces, ally_checking_pieces) = self.get_checking_pieces(player);

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

        if ally_checking_pieces.len() > 0 {
            let mut can_stop_checkmate = true;
            for attacker in ally_checking_pieces {
                if
                    !self.can_stop_check(
                        player,
                        self.squares[convert_position_1d(attacker.0, attacker.1)].get_type(),
                        attacker.0,
                        attacker.1
                    )
                {
                    can_stop_checkmate = false;
                    break;
                }
            }
            if !can_stop_checkmate {
                self.set_state(GameState::Won(player));
                return Ok(moves::MoveOutcome::Checkmate);
            }
        }

        self.squares[dst_index].set_last_move(self.turn, mv);
        self.turn += 1;

        self.set_state(GameState::Playing(other_player(player)));
        return Ok(moves::MoveOutcome::Continue);
    }

    pub fn execute_notation(
        &mut self,
        player: Option<game::Player>,
        notation: &str
    ) -> Result<moves::MoveOutcome, moves::MoveError> {
        // check if game still in playing state; extract current player
        let player = player.unwrap_or(match self.state {
            game::GameState::Playing(p) => p,
            game::GameState::Draw => {
                return Err(moves::MoveError::InvalidMove);
            }
            game::GameState::Won(_) => {
                return Err(moves::MoveError::InvalidMove);
            }
        });

        // handle "special" castling notation first
        if notation == "O-O" || notation == "O-O-O" {
            let castle_rank = match player {
                game::Player::White => 0,
                game::Player::Black => 7,
            };
            if notation == "O-O" {
                // king side castle
                let can_castle = self.can_king_castle(castle_rank);
                match can_castle {
                    Ok(_) => {
                        self.place_piece(player, pieces::PieceType::King, 6, castle_rank);
                        self.place_piece(player, pieces::PieceType::Rook, 5, castle_rank);
                        self.clear_square(4, castle_rank);
                        self.clear_square(7, castle_rank);
                        self.turn += 1;
                        match player {
                            game::Player::White => {
                                self.white_king = (6, 0);
                            }
                            game::Player::Black => {
                                self.black_king = (6, 7);
                            }
                        }
                        self.set_state(GameState::Playing(other_player(player)));
                        return Ok(moves::MoveOutcome::Continue);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            } else {
                // queen side castle
                match self.can_queen_castle(castle_rank) {
                    Ok(_) => {
                        self.place_piece(player, pieces::PieceType::King, 2, castle_rank);
                        self.place_piece(player, pieces::PieceType::Rook, 3, castle_rank);
                        self.clear_square(4, castle_rank);
                        self.clear_square(0, castle_rank);
                        self.turn += 1;
                        match player {
                            game::Player::White => {
                                self.white_king = (2, 0);
                            }
                            game::Player::Black => {
                                self.black_king = (2, 7);
                            }
                        }
                        self.set_state(GameState::Playing(other_player(player)));
                        return Ok(moves::MoveOutcome::Continue);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        } else {
            let piece_move = notation::parse_notation(self, &player, &notation);
            match piece_move {
                Ok(mv) => {
                    return self.execute_move(Some(player), mv);
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
    fn can_castle(&mut self, castle_rank: i8, rook_file: i8) -> Result<(), moves::MoveError> {
        if castle_rank != 0 && castle_rank != 7 {
            return Err(moves::MoveError::InvalidMove);
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
                    return Err(moves::MoveError::InvalidMove);
                }
            }

            // ensure that king will not pass through check during castle
            if rook_file == 0 {
                return match self.is_path_under_attack(defender, 2, castle_rank, 4, castle_rank) {
                    true => {
                        return Err(moves::MoveError::MoveIntoCheck);
                    }
                    false => Ok(()),
                };
            } else {
                return match self.is_path_under_attack(defender, 4, castle_rank, 6, castle_rank) {
                    true => {
                        return Err(moves::MoveError::MoveIntoCheck);
                    }
                    false => Ok(()),
                };
            }
        }
        return Err(moves::MoveError::InvalidMove);
    }

    // can a player castle king side?
    fn can_king_castle(&mut self, castle_rank: i8) -> Result<(), moves::MoveError> {
        return self.can_castle(castle_rank, 7);
    }

    // can a player castle queen side?
    fn can_queen_castle(&mut self, castle_rank: i8) -> Result<(), moves::MoveError> {
        return self.can_castle(castle_rank, 0);
    }

    pub fn is_path_under_attack(
        &mut self,
        defender: game::Player,
        path_start_file: i8,
        path_start_rank: i8,
        path_end_file: i8,
        path_end_rank: i8
    ) -> bool {
        let diff_y = (path_end_rank - path_start_rank).abs();
        let diff_x = (path_end_file - path_start_file).abs();

        // check if it's a valid path
        if diff_y == diff_x || diff_y == 0 || diff_x == 0 {
            for f in 0..8 {
                for r in 0..8 {
                    let attacker = &self.squares[convert_position_1d(f, r)];
                    let attacker_type = attacker.get_type();
                    if let Some(p) = attacker.get_player() {
                        let mut curr_file = path_start_file;
                        let mut curr_rank = path_start_rank;
                        while
                            (curr_file - path_end_file).abs() > 0 ||
                            (curr_rank - path_end_rank).abs() > 0
                        {
                            let tmp_mv = moves::PieceMove {
                                piece_type: attacker_type,
                                src_file: f,
                                src_rank: r,
                                dst_file: curr_file,
                                dst_rank: curr_rank,
                            };
                            if p != defender && self.piece_can_move(p, tmp_mv) {
                                return true;
                            }
                            if curr_file > path_end_file {
                                curr_file -= 1;
                            } else if curr_file < path_end_file {
                                curr_file += 1;
                            }
                            if curr_rank > path_end_rank {
                                curr_rank -= 1;
                            } else if curr_rank < path_end_rank {
                                curr_rank += 1;
                            }
                        }
                    }
                }
            }
        }
        return false;
    }

    fn can_stop_check(
        &mut self,
        attacker: game::Player,
        attacker_type: pieces::PieceType,
        attacker_file: i8,
        attacker_rank: i8
    ) -> bool {
        let king = match attacker {
            game::Player::White => self.black_king,
            game::Player::Black => self.white_king,
        };

        // test if the king can move out of check
        let king_moves = [
            (1, 1),
            (1, 0),
            (0, 1),
            (-1, 0),
            (0, -1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];
        for king_move in king_moves {
            if
                self.piece_can_move(other_player(attacker), PieceMove {
                    piece_type: pieces::PieceType::King,
                    src_file: king.0,
                    src_rank: king.1,
                    dst_file: king.0 + king_move.0,
                    dst_rank: king.1 + king_move.1,
                })
            {
                return true;
            }
        }

        // knight can't be blocked, don't check for it
        if attacker_type == pieces::PieceType::Knight {
            for f in 0..8 {
                for r in 0..8 {
                    let piece = &self.squares[convert_position_1d(f, r)];
                    if let Some(p) = piece.get_player() {
                        let tmp_mv = moves::PieceMove {
                            piece_type: self.squares[convert_position_1d(f, r)].get_type(),
                            src_file: f,
                            src_rank: r,
                            dst_file: attacker_file,
                            dst_rank: attacker_rank,
                        };
                        if p != attacker && self.piece_can_move(p, tmp_mv) {
                            return true;
                        }
                    }
                }
            }
            return false;
        }
        // the player inflicting check is "defending" the path between the checking piece and the enemy king
        return self.is_path_under_attack(attacker, attacker_file, attacker_rank, king.0, king.1);
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
