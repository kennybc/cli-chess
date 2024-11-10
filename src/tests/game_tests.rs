#[cfg(test)]
mod game_tests {
    use crate::board;

    fn import_game_from_file(file_path: &str) -> (Vec<String>, String) {
        let contents = std::fs
            ::read_to_string(file_path)
            .expect("Should have been able to read the file");

        // strip header (lines wrapped in [])
        let header_regex = regex::Regex::new(r"(?m)^\[.*\]\s*").unwrap();
        let stripped_contents = header_regex.replace_all(&contents, "");

        // remove move numbers from notation
        let result_regex = regex::Regex::new(r"\b(1-0|0-1|1/2-1/2)\b").unwrap();
        let result = result_regex.find(&stripped_contents).map(|mat| mat.as_str().to_string());
        let stripped_contents = result_regex.replace_all(&stripped_contents, "");

        let move_numbers_regex = regex::Regex::new(r"\d+\.\s*|1-0|0-1|1/2-1/2").unwrap();
        let stripped_contents = move_numbers_regex.replace_all(&stripped_contents, "");

        let move_regex = regex::Regex
            ::new(r"([KQRBN]?[a-h]?[1-8]?x?[a-h]?[1-8](=[QRBN])?[+#]?|O-O(?:-O)?)")
            .unwrap();

        let move_notations: Vec<String> = move_regex
            .find_iter(&stripped_contents)
            .map(|mat| mat.as_str().to_string())
            .collect();

        return (move_notations, result.unwrap());
    }

    #[test]
    fn test_all_games() {
        let paths = std::fs::read_dir("./games").unwrap();
        for path in paths {
            let mut board: board::Board = board::Board::new();
            board.reset_board();
            let game = import_game_from_file(path.unwrap().path().to_str().unwrap());
            for mv in game.0 {
                println!("checking: {mv}");
                let res = board.execute_notation(None, &mv);
                println!("{res:?}");
                assert_eq!(res.is_ok(), true);
            }
        }
    }
}
