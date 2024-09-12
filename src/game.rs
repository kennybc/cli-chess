#[derive(Debug, Copy, Clone)]
pub enum Player {
    White,
    Black,
}

pub enum GameState {
    Playing(Player),
    Won(Player),
    Draw,
}

pub fn other_player(p: Player) -> Player {
    match p {
        Player::White => Player::Black,
        Player::Black => Player::White,
    }
}
