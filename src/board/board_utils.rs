use super::{
    models::{Color, Piece, PieceType, Square},
    move_checking::square_utils::SquareIter,
    Board,
};

pub struct PlayerPieceIter<'a> {
    board: &'a Board,
    player: Color,
    square_iter: SquareIter,
}

impl<'a> PlayerPieceIter<'a> {
    pub fn new(board: &'a Board, player: Color) -> PlayerPieceIter<'a> {
        PlayerPieceIter {
            board,
            player,
            square_iter: SquareIter::new(),
        }
    }
}

// Iterates through a player's pieces
impl<'a> Iterator for PlayerPieceIter<'a> {
    type Item = (PieceType, Square);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(square) = self.square_iter.next() {
                if let Some(Piece(piece, owner)) = self.board.get_piece_at(square) {
                    if owner == self.player {
                        return Some((piece, square));
                    }
                }
            } else {
                return None;
            }
        }
    }
}
