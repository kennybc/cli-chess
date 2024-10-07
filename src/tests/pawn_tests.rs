#[cfg(test)]
mod pawn_tests {
    use crate::board;
    use crate::game;
    use crate::pieces;

    // test forward movement for white
    #[test]
    fn pawn_move_white() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();

        let pawn = &board.squares[board::convert_position_1d(0, 1)];
        assert_eq!(pawn.get_type(), pieces::PieceType::Pawn);
        assert_eq!(pawn.can_move(&board, 0, 2), true);
        assert_eq!(pawn.can_move(&board, 0, 3), true);
        assert_eq!(pawn.can_move(&board, 0, 4), false);
        assert_eq!(pawn.can_move(&board, 1, 2), false);
    }

    // test forward movement for black
    #[test]
    fn pawn_move_black() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();

        let pawn = &board.squares[board::convert_position_1d(0, 6)];
        assert_eq!(pawn.get_type(), pieces::PieceType::Pawn);
        assert_eq!(pawn.can_move(&board, 0, 5), true);
        assert_eq!(pawn.can_move(&board, 0, 4), true);
        assert_eq!(pawn.can_move(&board, 0, 3), false);
        assert_eq!(pawn.can_move(&board, 1, 5), false);
    }

    #[test]
    fn pawn_move_blocked() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();

        board.place_piece(game::Player::White, pieces::PieceType::Pawn, 0, 3);
        board.place_piece(game::Player::White, pieces::PieceType::Pawn, 0, 4);
        board.place_piece(game::Player::Black, pieces::PieceType::Pawn, 2, 3);
        board.place_piece(game::Player::Black, pieces::PieceType::Pawn, 2, 4);
        board.place_piece(game::Player::White, pieces::PieceType::Pawn, 4, 3);
        board.place_piece(game::Player::Black, pieces::PieceType::Pawn, 4, 4);
        let pawn1 = &board.squares[board::convert_position_1d(0, 3)];
        let pawn2 = &board.squares[board::convert_position_1d(2, 4)];
        let pawn3 = &board.squares[board::convert_position_1d(4, 3)];
        assert_eq!(pawn1.can_move(&board, 0, 4), false);
        assert_eq!(pawn2.can_move(&board, 2, 3), false);
        assert_eq!(pawn3.can_move(&board, 4, 4), false);
    }

    #[test]
    fn pawn_capture() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();

        board.place_piece(game::Player::White, pieces::PieceType::Pawn, 4, 3);
        let pawn = &board.squares[board::convert_position_1d(4, 3)];
        assert_eq!(pawn.get_type(), pieces::PieceType::Pawn);
        assert_eq!(pawn.can_move(&board, 4, 4), true);
        assert_eq!(pawn.can_move(&board, 3, 4), false);

        board.place_piece(game::Player::Black, pieces::PieceType::Pawn, 3, 4);
        let pawn = &board.squares[board::convert_position_1d(4, 3)];
        assert_eq!(pawn.get_type(), pieces::PieceType::Pawn);
        assert_eq!(pawn.can_move(&board, 4, 4), true);
        assert_eq!(pawn.can_move(&board, 3, 4), true);
    }

    // test en passant
    #[test]
    fn pawn_capture_en_passant() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();
        board.execute_notation(game::Player::White, "e4").unwrap();
        board.execute_notation(game::Player::White, "e5").unwrap();
        board.execute_notation(game::Player::Black, "d5").unwrap();

        let pawn = &board.squares[board::convert_position_1d(4, 4)];

        assert_eq!(pawn.get_type(), pieces::PieceType::Pawn);
        assert_eq!(pawn.can_move(&board, 3, 5), true);
        assert_eq!(pawn.can_move(&board, 3, 4), true);

        board.execute_notation(game::Player::Black, "a6").unwrap();
    }
}
