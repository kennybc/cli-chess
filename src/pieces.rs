#[derive(Debug, Copy, Clone)]
pub enum Piece {
    Empty,
    Pawn,
}

#[derive(Debug, Copy, Clone)]
pub struct Square {
    pub piece: Piece,
}
