use crate::square::Square;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChessMove {
    pub from: Square,
    pub to: Square
}
