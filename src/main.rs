use chess::model_utils::Opponent;
use chess::models::GameState;
use chess::player::{HumanPlayer, RandomPlayer};
use chess::run_game;
use uci::run_uci_engine;

mod chess;
mod uci;
mod search;

fn main() {
    run_uci_engine(&RandomPlayer);
}

fn run_test_game() {
    let human_player = HumanPlayer;
    let random_player = RandomPlayer;
    match run_game(&human_player, &random_player) {
        GameState::Mated(color) => println!("{} wins!", color.opponent()),
        GameState::Stalemate => println!("Stalemate!"),
        GameState::InProgress => unreachable!("Game should have ended"),
    }
}
