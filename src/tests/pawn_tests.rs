#[cfg(test)]
mod pawn_tests {
    use crate::board;
    use crate::game;
    use crate::pieces;

    // test forward movement
    #[test]
    fn pawn_move_white() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();
        let pawn = &board.squares[board::convert_position_1d(0, 1)];

        assert_eq!(pawn.get_type(), pieces::PieceType::Pawn);
        assert_eq!(pawn.can_move(&board, 0, 2), true);
        assert_eq!(pawn.can_move(&board, 0, 3), true);
        assert_eq!(pawn.can_move(&board, 0, 4), false);
    }

    #[test]
    fn pawn_move_black() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();
        let pawn = &board.squares[board::convert_position_1d(0, 6)];

        assert_eq!(pawn.get_type(), pieces::PieceType::Pawn);
        assert_eq!(pawn.can_move(&board, 0, 5), true);
        assert_eq!(pawn.can_move(&board, 0, 4), true);
        assert_eq!(pawn.can_move(&board, 0, 3), false);
    }

    #[test]
    fn pawn_move_capture() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();
        board.place_piece(game::Player::White, pieces::PieceType::Pawn, 4, 3);

        {
            let pawn = &board.squares[board::convert_position_1d(4, 3)];

            assert_eq!(pawn.get_type(), pieces::PieceType::Pawn);
            assert_eq!(pawn.can_move(&board, 4, 4), true);
            assert_eq!(pawn.can_move(&board, 3, 4), false);
        }

        {
            board.place_piece(game::Player::Black, pieces::PieceType::Pawn, 3, 4);
            let pawn = &board.squares[board::convert_position_1d(4, 3)];

            assert_eq!(pawn.get_type(), pieces::PieceType::Pawn);
            assert_eq!(pawn.can_move(&board, 4, 4), true);
            assert_eq!(pawn.can_move(&board, 3, 4), true);
        }
    }
}
