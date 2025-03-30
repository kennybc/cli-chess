mod game;
mod pieces;
mod board;
mod notation;
mod moves;

#[cfg(test)]
mod tests;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
