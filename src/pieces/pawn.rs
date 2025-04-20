use crate::board::shift;

fn white_single_push_targets(white_pawns: u64, empty: u64) -> u64 {
    return shift::shift_north(white_pawns) & empty;
}

fn white_double_push_targets(white_pawns: u64, empty: u64) -> u64 {
    const RANK4: u64 = 0x00000000ff000000;
    let single_push_targets = white_single_push_targets(white_pawns, empty);
    return shift::shift_north(single_push_targets) & empty & RANK4;
}

fn black_single_push_targets(black_pawns: u64, empty: u64) -> u64 {
    return shift::shift_south(black_pawns) & empty;
}

fn black_double_push_targets(black_pawns: u64, empty: u64) -> u64 {
    const RANK5: u64 = 0x000000ff00000000;
    let single_push_targets = black_single_push_targets(black_pawns, empty);
    return shift::shift_south(single_push_targets) & empty & RANK5;
}
