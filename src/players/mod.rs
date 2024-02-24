use crate::board::{models::LegalMove, Board};

pub mod human_player;
pub mod otus;
pub mod random_player;
pub trait ChessPlayer {
    fn make_move(&self, board: &Board) -> LegalMove;
}

pub struct HumanPlayer;
pub struct RandomPlayer;

pub struct Otus;
