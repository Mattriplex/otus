

use crate::{
    board::{Board},
    hashing::TranspTable,
    search::{
        eval::smart_eval,
        minimax::{search_minimax_threaded_cached},
    },
};

use super::{Otus, UciPlayer};

impl Default for Otus {
    fn default() -> Self {
        Self::new()
    }
}

impl Otus {
    // TODO make cache size, depth and other parameters configurable
    pub fn new() -> Self {
        Self {
            transp_table: TranspTable::new(2 << 24),
        }
    }
}

impl UciPlayer for Otus {
    fn propose_move(&mut self, board: &Board, rx: std::sync::mpsc::Receiver<()>) {
        search_minimax_threaded_cached(board, 6, smart_eval, &mut self.transp_table, rx);
    }
}
