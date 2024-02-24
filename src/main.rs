use otus::{
    board::{
        self,
        model_utils::Opponent,
        models::{Color, GameState},
        move_checking::{self, apply_legal_move},
        Board,
    },
    players::{ChessPlayer, HumanPlayer, Otus},
    search::minimax::search_minimax,
    uci::run_uci_engine,
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        match args[1].as_str() {
            "debug" => {
                run_test_game();
                return;
            }
            "perftest" => {
                let board = Board::default();
                println!("{}", search_minimax(&board, 5).to_move(&board));
                return;
            }
            _ => println!("Invalid argument"),
        }
    } else {
        run_uci_engine(&Otus);
    }
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
        board = apply_legal_move(&board, &m);
        match board.get_gamestate() {
            GameState::InProgress => (),
            gs => return gs,
        }
    }
}
