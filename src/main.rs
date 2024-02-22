use board::model_utils::Opponent;
use board::models::GameState;
use board::run_game;
use players::{HumanPlayer, RandomPlayer};
use uci::run_uci_engine;

mod board;
mod uci;
mod search;
mod players;

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
