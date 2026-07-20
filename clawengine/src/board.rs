use crate::{piece::{Color, Piece}, square::Square};

pub struct Board {
    pub squares: [Option<Piece>; 64],
    pub side_to_move: Color,
}

impl Board {
    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        self.squares[square as usize]
    }

    pub fn set_pieces(&mut self, square: Square, piece: Option<Piece>) {
        self.squares[square as usize] = piece;
    }
}
