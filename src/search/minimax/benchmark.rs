/**
 * The functions in this file are used for benchmarking and are simpler and less optimized on purpose.
*/


use crate::{
    board::{
        models::{GameState, LegalMove},
        move_generation::apply_legal_move,
        Board,
    },
    search::{eval::get_material_eval, minimax::get_noise},
};

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


