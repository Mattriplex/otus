use rand::Rng;

use crate::board::{get_legal_moves, models::Move, Board};

use super::{ChessPlayer, RandomPlayer};

impl ChessPlayer for RandomPlayer {
    fn make_move(&self, board: &Board) -> Move {
        let moves = get_legal_moves(board);
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..moves.len());
        moves[index].clone()
    }
}
