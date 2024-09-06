use crate::pieces;

#[derive(Debug)]
pub struct Board {
    squares: [pieces::Square; 64],
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [pieces::Square { piece: pieces::Piece::Empty }; 64],
        }
    }

    pub fn reset_board(&mut self) {
        self.squares = [pieces::Square { piece: pieces::Piece::Empty }; 64];
    }

    pub fn place_piece(&mut self, file: char, rank: u8) {
        let pos = self.convert_square(file, rank);
        self.squares[pos].piece = pieces::Piece::Pawn;
    }

    fn convert_square(&self, file: char, rank: u8) -> usize {
        (8 * (8 - rank) + self.convert_file(file)) as usize
    }

    fn convert_file(&self, c: char) -> u8 {
        let converted = (c as u8) - b'a';
        (c as u8) - b'a'
    }
}
