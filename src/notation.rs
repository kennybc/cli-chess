use crate::board;
use crate::pieces;
use crate::moves;
use regex::Regex;

fn get_piece_candidates(
    board: &board::Board,
    player: &pieces::Player,
    piece: &pieces::Piece,
    src_file: Option<i8>, // optional file/rank to discriminate attacker
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

pub fn parse_notation(board: &board::Board, notation: &str) -> Result<moves::Move, moves::Error> {
    let re = Regex::new(
        r"(?:(?P<piece_type>[kqrnKQRBN])?(?P<src_file>[a-h])?(?P<src_rank>[1-8])?(?P<capture>x)?(?P<dst_file>[a-h])(?P<dst_rank>[1-8])(?:=(?P<promotion>[qrbnQRBN]))?(?P<check>[+#])?)$"
    ).unwrap();

    // capture pattern matches and extract captured groups
    if let Some(caps) = re.captures(notation) {
        let mut piece = pieces::Piece::from_char(match caps.name("piece_type") {
            Some(c) => c.as_str().to_ascii_uppercase().chars().next().unwrap(),
            None => 'p',
        });
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
        let promotion = match caps.name("promotion") {
            Some(p) =>
                Some(
                    pieces::Piece::from_char(
                        p.as_str().to_ascii_uppercase().chars().next().unwrap()
                    )
                ),
            None => None,
        };
        let check = caps.name("check").map_or("", |m| m.as_str());

        /*return Ok(
            moves::PieceMove::new(piece, candidates[0].0, candidates[0].1, dst_file, dst_rank)
        );*/
        return Err(moves::Error::InvalidNotation);
    } else {
        return Err(moves::Error::InvalidNotation);
    }
}

// convert a file (a-h) to an integer (0-8)
fn convert_file(c: char) -> i8 {
    ((c as u8) - b'a') as i8
}
