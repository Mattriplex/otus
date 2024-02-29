use crate::{
    board::{models::LegalMove, Board}, hashing::TranspTable, uci::WorkerMessage
};

pub mod human_player;
pub mod otus;
pub mod random_player;
pub trait ChessPlayer {
    fn propose_move(&self, board: &Board) -> LegalMove;
}

pub trait UciPlayer {
    // propose_move should print `bestmove` to stdout and react to "stop" command
    fn propose_move(&mut self, board: &Board, rx: std::sync::mpsc::Receiver<()>);
}

pub struct HumanPlayer;
pub struct RandomPlayer;

pub struct Otus {
    transp_table: TranspTable
}


