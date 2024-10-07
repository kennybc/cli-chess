mod game;
mod pieces;
mod board;
mod notation;
mod moves;

#[cfg(test)]
mod tests;

fn main() {
    game::game_loop();
}
