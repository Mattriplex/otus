use crate::board::{models::Move, Board};

pub mod human_player;
pub mod random_player;
pub trait ChessPlayer {
    fn make_move(&self, board: &Board) -> Move;
}

pub struct HumanPlayer;
pub struct RandomPlayer;
