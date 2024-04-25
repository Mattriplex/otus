use otus::{
    board::{
        model_utils::ColorProps,
        models::{Color, GameState},
        move_checking::apply_legal_move,
        Board,
    },
    hashing::TranspTable,
    players::{ChessPlayer, HumanPlayer, RandomPlayer},
    search::{
        eval::smart_eval,
        minimax::{search_minimax_threaded_cached},
    },
    uci::UciEngine,
};

fn perftest() {
    let board = Board::default();
    let (_tx, rx) = std::sync::mpsc::channel();
    let mut transp_table = TranspTable::new(2 << 24);
    search_minimax_threaded_cached(&board, 6, smart_eval, &mut transp_table, rx);
    println!(
        "Transposition table occupancy: {}",
        transp_table.get_occupancy_factor()
    );
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        match args[1].as_str() {
            "debug" => {
                run_test_game();
            }
            "perftest" => {
                perftest();
            }
            _ => println!("Invalid argument"),
        }
    } else {
        UciEngine::new().run();
    }
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
