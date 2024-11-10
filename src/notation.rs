use crate::board;
use crate::game;
use crate::pieces;
use crate::moves;
use regex::Regex;

fn get_piece_candidates(
    board: &board::Board,
    player: &game::Player,
    piece_type: &pieces::PieceType,
    src_file: Option<i8>,
    src_rank: Option<i8>,
    dst_file: i8,
    dst_rank: i8
) -> Vec<(i8, i8)> {
    let mut candidates: Vec<(i8, i8)> = Vec::new();
    for f in 0..8 {
        if let Some(sf) = src_file {
            if f != sf {
                continue;
            }
        }
        for r in 0..8 {
            let candidate_square = &board.squares[board::convert_position_1d(f, r)];
            if let Some(p) = candidate_square.get_player() {
                if let Some(sr) = src_rank {
                    if r != sr {
                        continue;
                    }
                }
                if p == *player && candidate_square.get_type() == *piece_type {
                    if candidate_square.can_move(board, dst_file, dst_rank) {
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
) -> Result<moves::PieceMove, moves::MoveError> {
    let re = Regex::new(
        r"(?:(?P<piece_type>[kqrnKQRBN])?(?P<src_file>[a-h])?(?P<src_rank>[1-8])?(?P<capture>x)?(?P<dst_file>[a-h])(?P<dst_rank>[1-8])(?:=(?P<promotion>[qrbn]))?(?P<check>[+#])?)$"
    ).unwrap();

    // capture pattern matches and extract captured groups
    if let Some(caps) = re.captures(notation) {
        let piece_type = pieces::PieceType
            ::from_char(match caps.name("piece_type") {
                Some(c) => c.as_str().to_ascii_uppercase().chars().next().unwrap(),
                None => 'p',
            })
            .unwrap();
        let src_file = caps
            .name("src_file")
            .map_or(None, |m| Some(convert_file(m.as_str().chars().next().unwrap())));
        let src_rank = caps
            .name("src_rank")
            .map_or(None, |m| Some(m.as_str().parse::<i8>().unwrap() - 1));
        let dst_file = convert_file(
            caps.name("dst_file").unwrap().as_str().chars().next().unwrap()
        );
        let dst_rank = caps.name("dst_rank").unwrap().as_str().parse::<i8>().unwrap() - 1;
        let capture = caps.name("capture").map_or(false, |_| true);
        let promotion = caps.name("promotion").map_or("", |m| m.as_str());
        let check = caps.name("check").map_or("", |m| m.as_str());

        // get all potential pieces that could make this move
        let candidates = get_piece_candidates(
            &board,
            &player,
            &piece_type,
            src_file,
            src_rank,
            dst_file,
            dst_rank
        );
        if candidates.len() == 0 {
            return Err(moves::MoveError::InvalidMove);
        }
        if candidates.len() > 1 {
            for candidate in candidates {
                println!("{}:{}", candidate.0, candidate.1);
            }
            return Err(moves::MoveError::AmbiguousMove);
        }

        let piece = &board.squares[board::convert_position_1d(candidates[0].0, candidates[0].1)];
        if capture && !piece.can_attack(board, dst_file, dst_rank) {
            return Err(moves::MoveError::InvalidCapture);
        }
        if check == "todo" {
            return Err(moves::MoveError::InvalidCheck);
        }
        if promotion == "todo" {
            return Err(moves::MoveError::InvalidPromotion);
        }

        return Ok(moves::PieceMove {
            piece_type,
            src_file: candidates[0].0,
            src_rank: candidates[0].1,
            dst_file,
            dst_rank,
        });
    } else {
        return Err(moves::MoveError::InvalidNotation);
    }
}

// convert a file (a-h) to an integer (0-8)
fn convert_file(c: char) -> i8 {
    ((c as u8) - b'a') as i8
}
