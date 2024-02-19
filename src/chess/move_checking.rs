mod tests;

use crate::chess::{Board, Move};

pub fn apply_move(board: &Board, m: Move) -> Result<Board, String> {
    unimplemented!("apply_move")
}

pub fn is_move_legal(board: &Board, m: Move) -> bool {
    match apply_move(board, m) {
        Ok(_) => true,
        Err(_) => false,
    }
}
