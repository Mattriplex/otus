use chess::model_utils::Opponent;
use chess::models::GameState;
use chess::player::human_player::HumanPlayer;
use chess::run_game;

mod chess;

fn main() {
    let human_player = HumanPlayer;
    match run_game(&human_player, &human_player) {
        GameState::Mated(color) => println!("{} wins!", color.opponent()),
        GameState::Stalemate => println!("Stalemate!"),
        GameState::InProgress => unreachable!("Game should have ended"),
    }
}
