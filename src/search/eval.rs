use crate::board::{board_utils::PlayerPieceIter, models::{GameState, PieceType}, Board};


pub fn get_material_eval(board: &Board) -> f32 {
    if board.get_gamestate() == GameState::Mated(board.active_player) {
        return f32::MIN;
    }
    if board.get_gamestate() == GameState::Stalemate {
        return 0.0;
    }
    PlayerPieceIter::new(board, board.active_player)
        .map(|(piece, _)| match piece {
            PieceType::Pawn => 1.0,
            PieceType::Knight => 3.0,
            PieceType::Bishop => 3.0,
            PieceType::Rook => 5.0,
            PieceType::Queen => 9.0,
            PieceType::King => 0.0,
        })
        .sum()
}