use std::sync::mpsc;

use rand::Rng;

use crate::{
    board::{
        models::{GameState, LegalMove},
        move_checking::{apply_legal_move, is_king_in_check},
        Board,
    },
    uci::WorkerMessage,
};

use super::eval::get_material_eval;

fn get_noise() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(-0.1..0.1)
}

pub fn search_minimax(board: &Board, depth: u32) -> LegalMove {
    let moves = board.get_legal_moves(); // Assumption: this is never called in checkmated or stalemate position
    let mut best_move = moves[0].clone();
    let mut best_score = f32::MIN;
    for move_ in moves {
        let new_board = apply_legal_move(board, &move_);
        let score = -nega_max(&new_board, depth - 1) + get_noise(); // add noise to shuffle moves of equal value
        if score > best_score {
            best_score = score;
            best_move = move_;
        }
    }
    best_move
}

pub fn search_minimax_threaded(board: &Board, depth: u32, rx: mpsc::Receiver<()>) {
    let moves = board.get_legal_moves(); // Assumption: this is never called in checkmated or stalemate position
    let mut best_move = moves[0].clone();
    let mut best_score = f32::MIN;
    for move_ in moves {
        let new_board = apply_legal_move(&board, &move_);
        let score = -nega_max(&new_board, depth - 1) + get_noise(); // add noise to shuffle moves of equal value
        if score > best_score {
            best_score = score;
            best_move = move_;
        }
        if rx.try_recv().is_ok() {
            break;
        }
    }
    println!("bestmove {}", best_move.to_move(board))
}

fn nega_max(board: &Board, depth: u32) -> f32 {
    if depth == 0 {
        match board.get_gamestate() {
            GameState::Mated(_) => return f32::MIN,
            GameState::Stalemate => return 0.0,
            GameState::InProgress => return get_material_eval(board),
        }
    }
    let moves = board.get_legal_moves(); // Avoid calling get_gamestate because it would duplicate work from get_legal_moves()
    if moves.is_empty() {
        if is_king_in_check(board) {
            return f32::MIN; // mated
        } else {
            return 0.0; // stalemate
        }
    }
    let mut best_score = f32::MIN; // if no legal moves, return worst possible score TODO fix this for stalemate
    for move_ in moves {
        let new_board = apply_legal_move(board, &move_);
        let score = -nega_max(&new_board, depth - 1);
        if score > best_score {
            best_score = score;
        }
    }
    best_score
}

pub fn search_alpha_beta(board: &Board, depth: u32) -> LegalMove {
    let moves = board.get_legal_moves(); // Assumption: this is never called in checkmated or stalemate position
    let mut best_move = moves[0].clone();
    let mut best_score = f32::MIN;
    for move_ in moves {
        let new_board = apply_legal_move(board, &move_);
        let score = alpha_beta_min_rec(&new_board, depth - 1, best_score, f32::MAX) + get_noise(); // add noise to shuffle moves of equal value
        if score > best_score {
            best_score = score;
            best_move = move_;
        }
    }
    best_move
}

// alpha= minimum guaranteed score for me
// beta= maximum guaranteed score for opponent
fn alpha_beta_max_rec(board: &Board, depth: u32, mut alpha: f32, beta: f32) -> f32 {
    let gamestate = board.get_gamestate();
    if gamestate == GameState::Mated(board.active_player) {
        return f32::MIN;
    }
    if gamestate == GameState::Stalemate {
        return 0.0;
    }
    if depth == 0 {
        return get_material_eval(board);
    }
    let moves = board.get_legal_moves();
    for move_ in moves {
        let new_board = apply_legal_move(board, &move_);
        let score = alpha_beta_min_rec(&new_board, depth - 1, alpha, beta);
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }
    alpha
}

// alpha= minimum guaranteed score for opponent
// beta= maximum guaranteed score for me
fn alpha_beta_min_rec(board: &Board, depth: u32, alpha: f32, mut beta: f32) -> f32 {
    let gamestate = board.get_gamestate();
    if gamestate == GameState::Mated(board.active_player) {
        return f32::MIN;
    }
    if gamestate == GameState::Stalemate {
        return 0.0;
    }
    if depth == 0 {
        return get_material_eval(board);
    }
    let moves = board.get_legal_moves();
    for move_ in moves {
        let new_board = apply_legal_move(board, &move_);
        let score = -alpha_beta_max_rec(&new_board, depth - 1, alpha, beta);
        if score <= alpha {
            return alpha;
        }
        if score < beta {
            beta = score;
        }
    }
    alpha
}
