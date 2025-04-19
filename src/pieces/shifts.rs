const NOT_A_FILE: u64 = 0xfefefefefefefefe;
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

pub fn shift_north(board: u64) -> u64 {
    board << 8
}
pub fn shift_south(board: u64) -> u64 {
    board >> 8
}
pub fn shift_east(board: u64) -> u64 {
    (board & NOT_H_FILE) << 1
}
pub fn shift_northeast(board: u64) -> u64 {
    (board & NOT_H_FILE) << 9
}
pub fn shift_southeast(board: u64) -> u64 {
    (board & NOT_H_FILE) >> 7
}
pub fn shift_west(board: u64) -> u64 {
    (board & NOT_A_FILE) >> 1
}
pub fn shift_southwest(board: u64) -> u64 {
    (board & NOT_A_FILE) >> 9
}
pub fn shift_northwest(board: u64) -> u64 {
    (board & NOT_A_FILE) << 7
}
