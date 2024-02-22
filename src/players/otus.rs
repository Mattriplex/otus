use crate::{board::{models::Move, Board}, search::minimax::search_minimax};

use super::{ChessPlayer, Otus};

impl ChessPlayer for Otus {
    fn make_move(&self, board: &Board) -> Move {
        search_minimax(board, 4)
    }
}