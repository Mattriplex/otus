use std::thread;

use crate::{
    board::{
        models::{Move},
        move_checking::apply_move,
        Board,
    },
    players::{ChessPlayer, Otus, UciPlayer},
    search::{perft},
};

pub enum WorkerMessage {
    BestMove(Move),
    Info(String),
    Stop,
}

pub struct UciEngine {
    tx: std::sync::mpsc::Sender<()>,
    position: Board,
    computer_agent: Otus,
}

fn process_moves_list(initial_board: &Board, move_tokens: Vec<&str>) -> Board {
    let mut board = *initial_board;
    for token in move_tokens {
        let move_ = Move::from_uci_string(&board, token).expect("Invalid move syntax");
        board = apply_move(&board, &move_).expect("Illegal move");
    }
    board
}

impl Default for UciEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl UciEngine {
    pub fn new() -> Self {
        let (tx, _) = std::sync::mpsc::channel();
        Self {
            tx,
            position: Board::default(),
            computer_agent: Otus::new(),
        }
    }

    fn process_position_command(&mut self, arguments: Vec<&str>) {
        if arguments.is_empty() {
            return;
        }
        match arguments[0].to_lowercase().as_str() {
            "startpos" => {
                self.position = Board::default();
                if arguments.len() > 1 && arguments[1].to_lowercase() == "moves" {
                    self.position = process_moves_list(&self.position, arguments[2..].to_vec());
                }
            }
            "fen" => {
                let fen = arguments[1..7].join(" ");
                self.position = Board::from_fen(&fen).expect("Invalid FEN string");
                if arguments.len() > 7 && arguments[7].to_lowercase() == "moves" {
                    self.position = process_moves_list(&self.position, arguments[8..].to_vec());
                }
            }
            _ => {
                // ignore
            }
        }
    }

    fn process_go_command(&mut self, _arguments: Vec<&str>) {
        let (tx, rx) = std::sync::mpsc::channel();
        self.tx = tx;
        // TODO parse time control etc
        thread::scope(|s| {
            s.spawn(|| self.computer_agent.propose_move(&self.position, rx));
        });
    }

    fn process_command(&mut self, command: &str) {
        let tokens: Vec<&str> = command.split_whitespace().collect();
        if tokens.is_empty() {
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
            "position" => self.process_position_command(tokens[1..].to_vec()),
            "go" => {
                self.process_go_command(tokens[1..].to_vec());
            }
            "quit" => {
                std::process::exit(0);
            }
            "perft" => {
                if tokens.len() > 1 {
                    let depth = tokens[1].parse().expect("Invalid depth");
                    perft::perft(&mut self.position, depth);
                }
            }
            "stop" => {
                let _ = self.tx.send(()); // TODO if response from worker is too slow, add intermediate channel to cache latest best move
            }
            _ => {
                // stop, ponderhit, register, setoption, debug, ucinewgame
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            // commands are separated by a newline
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            for command in input.split('\n') {
                self.process_command(command);
            }
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
