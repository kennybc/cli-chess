use crate::pieces;

fn get_char(notation: &str, index: usize) -> char {
    return notation
        .chars()
        .nth(index)
        .expect("could not convert notation index to char")
        .to_ascii_lowercase();
}

pub fn parse_notation(notation: &str) -> (pieces::PieceType, u8, u8) {
    let mut pos_begin_index = 1;

    // get the piece type from notation
    let piece: pieces::PieceType = pieces::PieceType::from(get_char(notation, 0));

    if let pieces::PieceType::Pawn = piece {
        pos_begin_index = 0;
    }

    // get the destination file from notation
    let mut file: char = get_char(notation, pos_begin_index);
    if file == 'x' {
        pos_begin_index += 1;
        file = get_char(notation, pos_begin_index);
    }
    let file: u8 = convert_file(file);

    // get the destination rank from notation
    let rank: u8 =
        (
            get_char(notation, pos_begin_index + 1)
                .to_digit(10)
                .expect("invalid rank") as u8
        ) - 1;

    println!("piece: {piece:?}, file: {file}, rank: {rank}");

    return (piece, file, rank);
}

// convert a file (a-h) to an integer (0-8)
fn convert_file(c: char) -> u8 {
    (c as u8) - b'a'
}
