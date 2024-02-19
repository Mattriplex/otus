use chess::{run_game, HumanPlayer, Opponent};

mod chess;

fn main() {
    let human_player = HumanPlayer;
    match run_game(&human_player, &human_player) {
        chess::GameState::Mated(color) => println!("{} wins!", color.opponent()),
        chess::GameState::Stalemate => println!("Stalemate!"),
        chess::GameState::InProgress => unreachable!("Game should have ended"),
    }
}
