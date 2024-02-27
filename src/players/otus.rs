use crate::{
    board::{models::LegalMove, Board},
    search::minimax::search_minimax,
};

use super::{ChessPlayer, Otus};

impl ChessPlayer for Otus {
    fn propose_move(&self, board: &Board) -> LegalMove {
        search_minimax(board, 3)
    }
}
