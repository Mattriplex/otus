use super::{move_checking, Board, Move, Square};

pub mod human_player;
pub trait ChessPlayer {
    fn make_move(&self, board: &Board) -> Move;
}
