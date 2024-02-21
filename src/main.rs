use chess::model_utils::Opponent;
use chess::models::GameState;
use chess::player::{HumanPlayer, RandomPlayer};
use chess::run_game;

mod chess;

fn main() {
    let human_player = HumanPlayer;
    let random_player = RandomPlayer;
    match run_game(&human_player, &random_player) {
        GameState::Mated(color) => println!("{} wins!", color.opponent()),
        GameState::Stalemate => println!("Stalemate!"),
        GameState::InProgress => unreachable!("Game should have ended"),
    }
}
