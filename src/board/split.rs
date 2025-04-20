pub fn split_bitboard(mut board: u64) -> Vec<u64> {
    let mut result = Vec::new();

    while board != 0 {
        let lsb = board & board.wrapping_neg(); // isolate least significant bit
        result.push(lsb);
        board &= board - 1; // clear least significant bit
    }

    return result;
}
