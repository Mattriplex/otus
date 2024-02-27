use crate::{
    board::{models::LegalMove, Board},
    uci::WorkerMessage,
};

pub mod human_player;
pub mod otus;
pub mod random_player;
pub trait ChessPlayer {
    fn propose_move(&self, board: &Board) -> LegalMove;
}

pub struct HumanPlayer;
pub struct RandomPlayer;

pub struct Otus;
