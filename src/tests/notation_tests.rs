#[cfg(test)]
mod notation_tests {
    use crate::game;
    use crate::notation;
    use crate::board;

    #[test]
    fn pawn_move_notation() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();

        assert_eq!(notation::parse_notation(&board, &game::Player::White, "e3").is_err(), false);
        assert_eq!(notation::parse_notation(&board, &game::Player::White, "e4").is_err(), false);
        assert_eq!(notation::parse_notation(&board, &game::Player::White, "e5").is_err(), true);

        assert_eq!(notation::parse_notation(&board, &game::Player::Black, "d6").is_err(), false);
        assert_eq!(notation::parse_notation(&board, &game::Player::Black, "d5").is_err(), false);
        assert_eq!(notation::parse_notation(&board, &game::Player::Black, "d4").is_err(), true);
    }

    #[test]
    fn pawn_capture_notation() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();

        board.execute_move(game::Player::White, "e4").unwrap();
        board.execute_move(game::Player::Black, "d5").unwrap();
        assert_eq!(notation::parse_notation(&board, &game::Player::White, "exd5").is_err(), false);
        assert_eq!(notation::parse_notation(&board, &game::Player::Black, "dxe4").is_err(), false);
        assert_eq!(notation::parse_notation(&board, &game::Player::Black, "de4").is_err(), true);
    }
}
