#[cfg(test)]
mod bishop_tests {
    use crate::board;
    use crate::game;
    use crate::pieces;

    // test forward movement
    #[test]
    fn bishop_move() {
        let mut board: board::Board = board::Board::new();
        board.reset_board();

        {
            let bishopc1 = &board.squares[board::convert_position_1d(2, 0)];
            assert_eq!(bishopc1.get_type(), pieces::PieceType::Bishop);
            assert_eq!(bishopc1.can_move(&board, 3, 1), false);
        }

        board.clear_square(3, 1);
        {
            let bishopc1 = &board.squares[board::convert_position_1d(2, 0)];
            assert_eq!(bishopc1.get_type(), pieces::PieceType::Bishop);
            assert_eq!(bishopc1.can_move(&board, 3, 1), true);
            assert_eq!(bishopc1.can_move(&board, 3, 2), false);
            assert_eq!(bishopc1.can_move(&board, 4, 2), true);
            assert_eq!(bishopc1.can_move(&board, 5, 3), true);
            assert_eq!(bishopc1.can_move(&board, 6, 4), true);
            assert_eq!(bishopc1.can_move(&board, 7, 5), true);
        }

        board.place_piece(game::Player::White, pieces::PieceType::Pawn, 6, 4);
        {
            let bishopc1 = &board.squares[board::convert_position_1d(2, 0)];
            assert_eq!(bishopc1.get_type(), pieces::PieceType::Bishop);
            assert_eq!(bishopc1.can_move(&board, 3, 1), true);
            assert_eq!(bishopc1.can_move(&board, 3, 2), false);
            assert_eq!(bishopc1.can_move(&board, 4, 2), true);
            assert_eq!(bishopc1.can_move(&board, 5, 3), true);
            assert_eq!(bishopc1.can_move(&board, 6, 4), false);
            assert_eq!(bishopc1.can_move(&board, 7, 5), false);
        }

        board.place_piece(game::Player::Black, pieces::PieceType::Pawn, 6, 4);
        {
            let bishopc1 = &board.squares[board::convert_position_1d(2, 0)];
            assert_eq!(bishopc1.get_type(), pieces::PieceType::Bishop);
            assert_eq!(bishopc1.can_move(&board, 3, 1), true);
            assert_eq!(bishopc1.can_move(&board, 3, 2), false);
            assert_eq!(bishopc1.can_move(&board, 4, 2), true);
            assert_eq!(bishopc1.can_move(&board, 5, 3), true);
            assert_eq!(bishopc1.can_move(&board, 6, 4), true);
            assert_eq!(bishopc1.can_move(&board, 7, 5), false);
        }
    }
}
