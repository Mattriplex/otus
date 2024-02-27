use otus::{
    board::{
        model_utils::ColorProps,
        models::{Color, GameState},
        move_checking::apply_legal_move,
        Board,
    },
    players::{ChessPlayer, HumanPlayer, Otus},
    search::minimax::search_minimax,
    uci::UciEngine,
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        match args[1].as_str() {
            "debug" => {
                run_test_game();
            }
            "perftest" => {
                let board = Board::default();
                println!("{}", search_minimax(&board, 6).to_move(&board));
            }
            _ => println!("Invalid argument"),
        }
    } else {
        UciEngine::new().run();
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
            Color::White => white_player.propose_move(&board),
            Color::Black => black_player.propose_move(&board),
        };
        board = apply_legal_move(&board, &m);
        match board.get_gamestate() {
            GameState::InProgress => (),
            gs => return gs,
        }
    }
}
