use board::models::{Color, GameState};
use board::move_checking;
use board::{model_utils::Opponent, Board};
use players::{ChessPlayer, HumanPlayer, Otus, RandomPlayer};
use uci::run_uci_engine;

mod board;
mod players;
mod search;
mod uci;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 && args[1] == "debug" {
        run_test_game();
        return;
    }
    run_uci_engine(&Otus);
}

fn run_test_game() {
    let human_player = HumanPlayer;
    let otus = Otus;
    match run_game(&human_player, &otus) {
        GameState::Mated(color) => println!("{} wins!", color.opponent()),
        GameState::Stalemate => println!("Stalemate!"),
        GameState::InProgress => unreachable!("Game should have ended"),
    }
}

pub fn run_game(white_player: &dyn ChessPlayer, black_player: &dyn ChessPlayer) -> GameState {
    let mut board = Board::default();
    loop {
        let m = match board.active_player {
            Color::White => white_player.make_move(&board),
            Color::Black => black_player.make_move(&board),
        };
        match move_checking::apply_move(&board, &m) {
            Ok(new_board) => {
                board = new_board;
                match board.get_gamestate() {
                    GameState::InProgress => (),
                    gs => return gs,
                }
            }
            // Illegal move, game is forfeit
            Err(_) => return GameState::Mated(board.active_player),
        }
    }
}
