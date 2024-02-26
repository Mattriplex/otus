use rand::Rng;

use crate::board::{
    models::{LegalMove},
    Board,
};

use super::{ChessPlayer, RandomPlayer};

impl ChessPlayer for RandomPlayer {
    fn propose_move(&self, board: &Board) -> LegalMove {
        let moves = board.get_legal_moves();
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..moves.len());
        moves[index].clone()
    }
}
