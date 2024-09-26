#[cfg(test)]
mod king_tests {
    use crate::board;
    use crate::game;
    use crate::pieces;

    #[test]
    fn king_move() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();

        board.clear_square(4, 1);
        let king = &board.squares[board::convert_position_1d(4, 0)];
        assert_eq!(king.get_type(), pieces::PieceType::King);
        assert_eq!(king.can_move(&board, 4, 1), true);
    }

    // ensure king cannot move into a square occupied by an ally
    #[test]
    fn king_move_blocked() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();

        let king = &board.squares[board::convert_position_1d(4, 0)];
        assert_eq!(king.get_type(), pieces::PieceType::King);
        assert_eq!(king.can_move(&board, 4, 1), false);
    }

    // ensure king can move into a square occupied by an enemy
    #[test]
    fn king_capture() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();

        board.place_piece(game::Player::Black, pieces::PieceType::Knight, 4, 1);
        let king = &board.squares[board::convert_position_1d(4, 0)];
        assert_eq!(king.get_type(), pieces::PieceType::King);
        assert_eq!(king.can_move(&board, 4, 1), true);
    }

    // ensure cannot capture a defended enemy
    #[test]
    fn king_capture_defended() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();

        board.place_piece(game::Player::Black, pieces::PieceType::Knight, 4, 1);
        board.place_piece(game::Player::Black, pieces::PieceType::Knight, 2, 2);
        let king = &board.squares[board::convert_position_1d(4, 0)];
        assert_eq!(king.get_type(), pieces::PieceType::King);
        assert_eq!(king.can_move(&board, 4, 1), false);
    }

    // ensure king cannot move into check
    #[test]
    fn king_move_defended() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();

        board.clear_square(4, 1);
        board.place_piece(game::Player::Black, pieces::PieceType::Knight, 2, 2);
        let king = &board.squares[board::convert_position_1d(4, 0)];
        assert_eq!(king.get_type(), pieces::PieceType::King);
        assert_eq!(king.can_move(&board, 4, 1), false);
    }
}
