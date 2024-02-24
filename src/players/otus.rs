use crate::{
    board::{
        models::{LegalMove, Move},
        Board,
    },
    search::minimax::search_minimax,
};

use super::{ChessPlayer, Otus};

impl ChessPlayer for Otus {
    fn make_move(&self, board: &Board) -> LegalMove {
        search_minimax(board, 3)
    }
}
