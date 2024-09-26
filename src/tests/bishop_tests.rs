#[cfg(test)]
mod bishop_tests {
    use crate::board;
    use crate::game;
    use crate::pieces;

    // ensure bishop can move freely along diagonal
    #[test]
    fn bishop_move() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();

        board.clear_square(3, 1);
        let bishop = &board.squares[board::convert_position_1d(2, 0)];
        assert_eq!(bishop.get_type(), pieces::PieceType::Bishop);
        assert_eq!(bishop.can_move(&board, 3, 1), true);
        assert_eq!(bishop.can_move(&board, 3, 2), false);
        assert_eq!(bishop.can_move(&board, 4, 2), true);
        assert_eq!(bishop.can_move(&board, 5, 3), true);
        assert_eq!(bishop.can_move(&board, 6, 4), true);
        assert_eq!(bishop.can_move(&board, 6, 5), false);
        assert_eq!(bishop.can_move(&board, 7, 5), true);
    }

    // ensure bishop cannot move past any piece or into ally occupied square
    #[test]
    fn bishop_move_blocked() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();

        let bishop = &board.squares[board::convert_position_1d(2, 0)];
        assert_eq!(bishop.get_type(), pieces::PieceType::Bishop);
        assert_eq!(bishop.can_move(&board, 3, 1), false);

        board.clear_square(3, 1);
        let bishop = &board.squares[board::convert_position_1d(2, 0)];
        assert_eq!(bishop.get_type(), pieces::PieceType::Bishop);
        assert_eq!(bishop.can_move(&board, 3, 1), true);
        assert_eq!(bishop.can_move(&board, 4, 2), true);
        assert_eq!(bishop.can_move(&board, 5, 3), true);
        assert_eq!(bishop.can_move(&board, 6, 4), true);
        assert_eq!(bishop.can_move(&board, 7, 5), true);

        board.place_piece(game::Player::White, pieces::PieceType::Pawn, 6, 4);
        let bishop = &board.squares[board::convert_position_1d(2, 0)];
        assert_eq!(bishop.get_type(), pieces::PieceType::Bishop);
        assert_eq!(bishop.can_move(&board, 3, 1), true);
        assert_eq!(bishop.can_move(&board, 3, 2), false);
        assert_eq!(bishop.can_move(&board, 4, 2), true);
        assert_eq!(bishop.can_move(&board, 5, 3), true);
        assert_eq!(bishop.can_move(&board, 6, 4), false);
        assert_eq!(bishop.can_move(&board, 7, 5), false);

        board.place_piece(game::Player::Black, pieces::PieceType::Pawn, 6, 4);
        let bishop = &board.squares[board::convert_position_1d(2, 0)];
        assert_eq!(bishop.get_type(), pieces::PieceType::Bishop);
        assert_eq!(bishop.can_move(&board, 3, 1), true);
        assert_eq!(bishop.can_move(&board, 3, 2), false);
        assert_eq!(bishop.can_move(&board, 4, 2), true);
        assert_eq!(bishop.can_move(&board, 5, 3), true);
        assert_eq!(bishop.can_move(&board, 6, 4), true);
        assert_eq!(bishop.can_move(&board, 7, 5), false);
    }

    // ensure bishop can capture enemy piece
    #[test]
    fn bishop_capture() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();

        board.clear_square(3, 1);
        board.place_piece(game::Player::Black, pieces::PieceType::Pawn, 6, 4);
        board.place_piece(game::Player::Black, pieces::PieceType::Pawn, 7, 5);
        let bishop = &board.squares[board::convert_position_1d(2, 0)];
        assert_eq!(bishop.get_type(), pieces::PieceType::Bishop);
        assert_eq!(bishop.can_move(&board, 6, 4), true);
        assert_eq!(bishop.can_move(&board, 7, 5), false);
    }
}
