use super::{move_checking, Board, Move, Pos};

pub mod human_player;
pub trait ChessPlayer {
    fn make_move(&self, board: &Board) -> Move;
}
