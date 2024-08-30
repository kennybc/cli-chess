use std::io;

fn main() {
    println!("Enter your move:");

    let mut next_move = String::new();
    io::stdin().read_line(&mut next_move).expect("failed to read line");
}
