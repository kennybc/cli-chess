#[cfg(test)]
mod tests {
    use crate::board;
    use crate::game;
    use crate::pieces;

    // test forward movement
    #[test]
    fn white_move() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();

        assert_eq!(
            board.squares[board::convert_position_1d(0, 1)].get_type(),
            pieces::PieceType::Pawn
        );

        assert_eq!(
            board.squares[board::convert_position_1d(0, 1)].can_move(&board, pieces::PiecePosition {
                player: game::Player::White,
                file: 0,
                rank: 2,
            }),
            true
        );
        assert_eq!(
            board.squares[board::convert_position_1d(0, 1)].can_move(&board, pieces::PiecePosition {
                player: game::Player::White,
                file: 0,
                rank: 3,
            }),
            true
        );
        assert_eq!(
            board.squares[board::convert_position_1d(0, 1)].can_move(&board, pieces::PiecePosition {
                player: game::Player::White,
                file: 0,
                rank: 4,
            }),
            false
        );
    }

    #[test]
    fn black_move() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();

        assert_eq!(
            board.squares[board::convert_position_1d(0, 6)].get_type(),
            pieces::PieceType::Pawn
        );

        assert_eq!(
            board.squares[board::convert_position_1d(0, 6)].can_move(&board, pieces::PiecePosition {
                player: game::Player::Black,
                file: 0,
                rank: 5,
            }),
            true
        );
        assert_eq!(
            board.squares[board::convert_position_1d(0, 6)].can_move(&board, pieces::PiecePosition {
                player: game::Player::Black,
                file: 0,
                rank: 4,
            }),
            true
        );
        assert_eq!(
            board.squares[board::convert_position_1d(0, 6)].can_move(&board, pieces::PiecePosition {
                player: game::Player::Black,
                file: 0,
                rank: 3,
            }),
            false
        );
    }
}
