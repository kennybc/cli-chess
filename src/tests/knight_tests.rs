#[cfg(test)]
mod knight_tests {
    use crate::board;
    use crate::pieces;

    // test knight L shaped movement
    #[test]
    fn knight_move() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();

        let knightb1 = &board.squares[board::convert_position_1d(1, 0)];
        assert_eq!(knightb1.get_type(), pieces::PieceType::Knight);
        assert_eq!(knightb1.can_move(&board, 0, 2), true);
        assert_eq!(knightb1.can_move(&board, 1, 2), false);
        assert_eq!(knightb1.can_move(&board, 2, 2), true);
        assert_eq!(knightb1.can_move(&board, 2, 1), false);

        let knightg8 = &board.squares[board::convert_position_1d(6, 7)];
        assert_eq!(knightg8.get_type(), pieces::PieceType::Knight);
        assert_eq!(knightg8.can_move(&board, 7, 5), true);
        assert_eq!(knightg8.can_move(&board, 6, 5), false);
        assert_eq!(knightg8.can_move(&board, 5, 5), true);
        assert_eq!(knightg8.can_move(&board, 5, 6), false);
    }

    // ensure knight cannot move into ally occupied square
    #[test]
    fn knight_move_blocked() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();

        let knightb1 = &board.squares[board::convert_position_1d(1, 0)];
        assert_eq!(knightb1.get_type(), pieces::PieceType::Knight);
        assert_eq!(knightb1.can_move(&board, 3, 1), false);

        let knightg8 = &board.squares[board::convert_position_1d(6, 7)];
        assert_eq!(knightg8.get_type(), pieces::PieceType::Knight);
        assert_eq!(knightg8.can_move(&board, 4, 6), false);
    }
}
