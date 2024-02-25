use crate::board::{
    models::{Piece, PieceType},
    move_checking::square_utils::SquareIter,
    Board,
};

pub fn get_material_eval(board: &Board) -> f32 {
    let mut material_balance = 0.0;
    for square in SquareIter::new() {
        if let Some(Piece(piece, owner)) = board.get_piece_at(square) {
            let value = match piece {
                PieceType::Pawn => 1.0,
                PieceType::Knight => 3.0,
                PieceType::Bishop => 3.0,
                PieceType::Rook => 5.0,
                PieceType::Queen => 9.0,
                PieceType::King => continue,
            };
            material_balance += if owner == board.active_player {
                value
            } else {
                -value
            };
        }
    }
    material_balance
}
