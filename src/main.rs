mod game;
mod pieces;
mod board;
mod notation;

#[cfg(test)]
mod tests;

fn main() {
    game::game_loop();
}
