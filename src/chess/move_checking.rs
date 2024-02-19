#[cfg(test)]
mod tests;

use crate::chess::{Board, Move};

use super::GameState;

// TODO: switch active player
pub fn apply_move(board: &Board, move_: &Move) -> Result<Board, String> {
    unimplemented!("apply_move")
}

pub fn is_move_legal(board: &Board, move_: &Move) -> bool {
    match apply_move(board, move_) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn get_gamestate(board: &Board) -> GameState {
    unimplemented!("get_gamestate")
}
