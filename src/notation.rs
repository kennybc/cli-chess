use crate::board;
use crate::game;
use crate::pieces;

fn get_char(notation: &str, index: usize) -> char {
    return notation
        .chars()
        .nth(index)
        .expect("could not convert notation index to char")
        .to_ascii_lowercase();
}

fn get_piece_candidates(
    board: &board::Board,
    player: &game::Player,
    piece_type: &pieces::PieceType,
    file: i8,
    rank: i8
) -> Vec<(i8, i8)> {
    let mut candidates: Vec<(i8, i8)> = Vec::new();
    for f in 0..8 {
        for r in 0..8 {
            let candidate_square = &board.squares[board::convert_position_1d(f, r)];
            if let Some(p) = candidate_square.get_player() {
                if p == *player && candidate_square.get_type() == *piece_type {
                    if
                        candidate_square.can_move(board, file, rank) ||
                        candidate_square.can_capture(board, file, rank)
                    {
                        candidates.push((f, r));
                    }
                }
            }
        }
    }
    return candidates;
}

pub fn parse_notation(
    board: &board::Board,
    player: &game::Player,
    notation: &str
) -> Result<pieces::PieceMove, ()> {
    let mut pos_begin_index = 1;

    // get the piece type from notation
    let piece_type: pieces::PieceType = pieces::PieceType::from(get_char(notation, 0));

    if pieces::PieceType::Pawn == piece_type {
        pos_begin_index = 0;
    }

    // get the destination file from notation
    let mut file: char = get_char(notation, pos_begin_index);
    if file == 'x' {
        pos_begin_index += 1;
        file = get_char(notation, pos_begin_index);
    }
    let file: i8 = convert_file(file);

    // get the destination rank from notation
    let rank: i8 = (get_char(notation, pos_begin_index + 1)
        .to_digit(10)
        .expect("invalid rank") - 1) as i8;

    println!("piece: {piece_type:?}, file: {file}, rank: {rank}");

    // get all potential pieces that could make this move
    let candidates = get_piece_candidates(board, player, &piece_type, file, rank);
    if candidates.len() == 0 {
        println!("no valid candidates");
        return Err(());
    }
    for candidate in candidates {
        println!("candidate: {candidate:?}");
        // todo
    }

    return Ok(pieces::PieceMove {
        piece_type,
        src_file: file,
        src_rank: rank,
        dst_file: file,
        dst_rank: rank,
    });
}

// convert a file (a-h) to an integer (0-8)
fn convert_file(c: char) -> i8 {
    ((c as u8) - b'a') as i8
}
