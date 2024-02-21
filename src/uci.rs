use crate::chess::{models::Move, move_checking::apply_move, player::ChessPlayer, Board};

fn parse_moves_list(move_tokens: Vec<&str>) -> Vec<Move> {
    move_tokens
        .iter()
        .map(|move_| Move::from_uci_string(move_).expect("Invalid move syntax"))
        .collect()
}

fn process_position_command(arguments: Vec<&str>, board: &mut Board) {
    if arguments.len() == 0 {
        return;
    }
    match arguments[0].to_lowercase().as_str() {
        "startpos" => {
            *board = Board::default();
            if arguments.len() > 1 && arguments[1].to_lowercase() == "moves" {
                let moves = parse_moves_list(arguments[2..].to_vec());
                for move_ in moves {
                    *board = apply_move(board, &move_).expect("Illegal move");
                }
            }
        }
        "fen" => {
            let fen = arguments[1..7].join(" ");
            *board = Board::from_fen(&fen).expect("Invalid FEN string");
            if arguments.len() > 7 && arguments[7].to_lowercase() == "moves" {
                let moves = parse_moves_list(arguments[8..].to_vec());
                for move_ in moves {
                    *board = apply_move(board, &move_).expect("Illegal move");
                }
            }
        }
        _ => {
            // ignore
        }
    }
}

pub fn process_go_command(arguments: Vec<&str>, player: &impl ChessPlayer, board: &mut Board) {
    // TODO parse time control etc
    let move_ = player.make_move(board);
    println!("bestmove {}", move_); // TODO: Display may not be UCI-compliant
}

pub fn process_command(command: &str, board: &mut Board, player: &impl ChessPlayer) {
    let tokens: Vec<&str> = command.split_whitespace().collect();
    if tokens.len() == 0 {
        return;
    }
    match tokens[0].to_lowercase().as_str() {
        "uci" => {
            println!("id name Otus");
            println!("id author Matthias Roshardt");
            println!("uciok");
        }
        "isready" => {
            println!("readyok");
        }
        "position" => process_position_command(tokens[1..].to_vec(), board),
        "go" => {
            process_go_command(tokens[1..].to_vec(), player, board);
        }
        "quit" => {
            std::process::exit(0);
        }
        _ => {
            // stop, ponderhit, register, setoption, debug, ucinewgame
        }
    }
}

pub fn run_uci_engine(player: &impl ChessPlayer) {
    let mut board: Board = Board::default();
    loop {
        // commands are separated by a newline
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        for command in input.split('\n') {
            process_command(command, &mut board, player);
        }
    }
}

/*
Output commands:
 - id: response to 'uci'
 - uciok: response to 'uci'
 - readyok: response to 'isready'
 - bestmove: response to 'go'
 - info: response to 'go', should be sent before bestmove
    - should include: hashfull, nps
    - can send arbitrary string with 'string' (needs to be at the end of the line)
 - option: response to 'uci', send after 'id'
 */
