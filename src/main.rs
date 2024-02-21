use chess::models::GameState;
use chess::move_checking::get_legal_moves;
use chess::player::human_player::HumanPlayer;
use chess::{run_game, Board, Opponent};

mod chess;

fn main() {
    let human_player = HumanPlayer;
    let board =
        Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
    for mv in get_legal_moves(&board) {
        println!("{}", mv);
    }

    match run_game(&human_player, &human_player) {
        GameState::Mated(color) => println!("{} wins!", color.opponent()),
        GameState::Stalemate => println!("Stalemate!"),
        GameState::InProgress => unreachable!("Game should have ended"),
    }
}
