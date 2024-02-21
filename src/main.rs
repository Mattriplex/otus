use chess::move_checking::get_legal_moves;
use chess::{run_game, Board, HumanPlayer, Opponent};

mod chess;

fn main() {
    let human_player = HumanPlayer;
    let board =
        Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
    for mv in get_legal_moves(&board) {
        println!("{}", mv);
    }

    match run_game(&human_player, &human_player) {
        chess::GameState::Mated(color) => println!("{} wins!", color.opponent()),
        chess::GameState::Stalemate => println!("Stalemate!"),
        chess::GameState::InProgress => unreachable!("Game should have ended"),
    }
}
