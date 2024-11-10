#[cfg(test)]
mod game_tests {
    use crate::board;
    use crate::game;

    fn import_game_from_file(file_path: &str) -> (Vec<String>, String) {
        let contents = std::fs
            ::read_to_string(file_path)
            .expect("Should have been able to read the file");

        // extract result from header
        let result_regex = regex::Regex::new(r#"\[Result\s*"([01/2-]+)"\]"#).unwrap();
        let result = result_regex
            .captures(&contents)
            .map(|caps| caps.get(1).map(|m| m.as_str().to_string()))
            .unwrap();

        // strip header (lines wrapped in [])
        let header_regex = regex::Regex::new(r"(?m)^\[.*\]\s*").unwrap();
        let stripped_contents = header_regex.replace_all(&contents, "");

        // remove move numbers from notation
        let move_numbers_regex = regex::Regex::new(r"\d+\.\s*").unwrap();
        let stripped_contents = move_numbers_regex.replace_all(&stripped_contents, "");

        let move_regex = regex::Regex
            ::new(
                r"(1-0|0-1|1/2-1/2)|([KQRBN]?[a-h]?[1-8]?x?[a-h]?[1-8](=[QRBN])?[+#]?|O-O(?:-O)?)"
            )
            .unwrap();

        let move_notations: Vec<String> = move_regex
            .find_iter(&stripped_contents)
            .map(|mat| mat.as_str().to_string())
            .collect();

        return (move_notations, result.unwrap());
    }

    fn result_to_state(result: &str) -> game::GameState {
        println!("{result}");
        match result {
            "1-0" => game::GameState::Won(game::Player::White),
            "0-1" => game::GameState::Won(game::Player::Black),
            _ => game::GameState::Draw,
        }
    }

    #[test]
    fn test_all_games() {
        let paths = std::fs::read_dir("./games").unwrap();
        for path in paths {
            let mut board: board::Board = board::Board::new();
            board.reset_board();
            let game = import_game_from_file(path.unwrap().path().to_str().unwrap());
            for mv in game.0 {
                let res = board.execute_notation(None, &mv);
                assert_eq!(res.is_ok(), true);
            }

            assert_eq!(board.get_state(), &result_to_state(&game.1));
        }
    }
}
