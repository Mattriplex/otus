use std::sync::mpsc;

use rand::Rng;

use crate::{
    board::{
        models::{GameState, LegalMove},
        move_checking::{apply_legal_move, is_king_in_check},
        Board,
    },
    hashing::{get_zobrist_hash, update_zobrist_hash, TranspEntry, TranspTable},
};

use super::eval::get_material_eval;

fn get_noise() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(-0.1..0.1)
}

struct SearchResult {
    eval: f32,
    nodes_searched: u64,
}

pub fn search_minimax_threaded_cached(
    board: &Board,
    depth: u8,
    eval_fn: fn(&Board) -> f32,
    trans_table: &mut TranspTable,
    rx: mpsc::Receiver<()>,
) {
    let moves = board.get_legal_moves(); // Assumption: this is never called in checkmated or stalemate position
    let mut best_move = moves[0].clone();
    let mut best_score = f32::MIN;
    let initial_hash = get_zobrist_hash(board);
    let mut nodes_searched = 0;
    for move_ in moves {
        let new_board = apply_legal_move(board, &move_);
        let time = std::time::Instant::now();
        let result = nega_max_cached(
            &new_board,
            depth - 1,
            f32::MIN,
            f32::MAX,
            eval_fn,
            trans_table,
            update_zobrist_hash(board, initial_hash, &move_),
        );
        let time_elapsed = time.elapsed().as_micros();
        nodes_searched += result.nodes_searched;
        println!(
            "info nodes {} nps {}",
            nodes_searched,
            (result.nodes_searched as f64 / time_elapsed as f64 * 1_000_000.0) as i64
        );
        let score = -result.eval + get_noise(); // add noise to shuffle moves of equal value
        if score > best_score {
            best_score = score;
            best_move = move_;
        }
        if rx.try_recv().is_ok() {
            break;
        }
    }
    println!("bestmove {}", best_move.to_move(board).to_uci_string(board))
}

pub fn search_minimax_cached(
    board: &Board,
    depth: u8,
    eval_fn: fn(&Board) -> f32,
    trans_table: &mut TranspTable,
) -> LegalMove {
    let moves = board.get_legal_moves(); // Assumption: this is never called in checkmated or stalemate position
    let mut best_move = moves[0].clone();
    let mut best_score = f32::MIN;
    let initial_hash = get_zobrist_hash(board);
    for move_ in moves {
        let new_board = apply_legal_move(board, &move_);
        let result = nega_max_cached(
            &new_board,
            depth - 1,
            f32::MIN,
            f32::MAX,
            eval_fn,
            trans_table,
            update_zobrist_hash(board, initial_hash, &move_),
        );
        let score = -result.eval + get_noise(); // add noise to shuffle moves of equal value
        if score > best_score {
            best_score = score;
            best_move = move_;
        }
    }
    best_move
}

fn get_cached_eval(board: &Board, board_hash: u64, move_: &LegalMove, cache: &TranspTable) -> f32 {
    let hash = update_zobrist_hash(board, board_hash, move_);
    match cache.get(hash) {
        Some(entry) => entry.value,
        None => f32::MAX,
    }
}

fn nega_max_cached(
    board: &Board,
    depth: u8,
    mut alpha: f32,
    beta: f32,
    eval_fn: fn(&Board) -> f32,
    trans_table: &mut TranspTable,
    board_hash: u64,
) -> SearchResult {
    let cache_entry = trans_table.get(board_hash);
    if let Some(entry) = cache_entry {
        if entry.depth >= depth {
            return SearchResult {
                eval: entry.value,
                nodes_searched: 0,
            };
        }
    }
    if depth == 0 {
        let eval = match board.get_gamestate() {
            GameState::Mated(_) => f32::MIN,
            GameState::Stalemate => 0.0,
            GameState::InProgress => eval_fn(board),
        };
        trans_table.put(
            board_hash,
            TranspEntry {
                depth: 0,
                value: eval,
            },
        ); // TODO experiment if this is actually faster
        return SearchResult {
            eval,
            nodes_searched: 1,
        };
    }
    let mut moves = board.get_legal_moves(); // Avoid calling get_gamestate because it would duplicate work from get_legal_moves()
    if moves.is_empty() {
        let eval = if is_king_in_check(board) {
            f32::MIN // mated
        } else {
            0.0 // stalemate
        };
        trans_table.put(
            board_hash,
            TranspEntry {
                depth: 0,
                value: eval,
            },
        ); // TODO experiment if this is actually faster
        return SearchResult {
            eval,
            nodes_searched: 1,
        };
    }
    // move ordering
    moves.sort_unstable_by(|a, b| {
        let eval_a = get_cached_eval(board, board_hash, a, trans_table);
        let eval_b = get_cached_eval(board, board_hash, b, trans_table);
        eval_a.partial_cmp(&eval_b).unwrap() // want to sort valuations in ascending order, these are opponent evals, opps worst situation is my best move
    });
    let mut nodes_searched = 0;
    for move_ in moves {
        let new_board = apply_legal_move(board, &move_);
        let result = nega_max_cached(
            &new_board,
            depth - 1,
            -beta,
            -alpha,
            eval_fn,
            trans_table,
            update_zobrist_hash(board, board_hash, &move_),
        );
        let score = -result.eval;
        if score >= beta {
            return SearchResult {
                eval: beta,
                nodes_searched,
            };
        }
        nodes_searched += result.nodes_searched;
        if score > alpha {
            alpha = score;
        }
    }
    trans_table.put(
        board_hash,
        TranspEntry {
            depth,
            value: alpha,
        },
    );
    SearchResult {
        eval: alpha,
        nodes_searched,
    }
}

pub fn search_minimax(board: &Board, depth: u32, eval_fn: fn(&Board) -> f32) -> LegalMove {
    let moves = board.get_legal_moves(); // Assumption: this is never called in checkmated or stalemate position
    let mut best_move = moves[0].clone();
    let mut best_score = f32::MIN;
    for move_ in moves {
        let new_board = apply_legal_move(board, &move_);
        let score = -nega_max(&new_board, depth - 1, eval_fn) + get_noise(); // add noise to shuffle moves of equal value
        if score > best_score {
            best_score = score;
            best_move = move_;
        }
    }
    best_move
}

pub fn search_minimax_threaded(
    board: &Board,
    depth: u32,
    eval_fn: fn(&Board) -> f32,
    rx: mpsc::Receiver<()>,
) {
    let moves = board.get_legal_moves(); // Assumption: this is never called in checkmated or stalemate position
    let mut best_move = moves[0].clone();
    let mut best_score = f32::MIN;
    for move_ in moves {
        let new_board = apply_legal_move(board, &move_);
        let score = -nega_max(&new_board, depth - 1, eval_fn) + get_noise(); // add noise to shuffle moves of equal value
        if score > best_score {
            best_score = score;
            best_move = move_;
        }
        if rx.try_recv().is_ok() {
            break;
        }
    }
    println!("bestmove {}", best_move.to_move(board).to_uci_string(board))
}

fn nega_max(board: &Board, depth: u32, eval_fn: fn(&Board) -> f32) -> f32 {
    if depth == 0 {
        match board.get_gamestate() {
            GameState::Mated(_) => return f32::MIN,
            GameState::Stalemate => return 0.0,
            GameState::InProgress => return eval_fn(board),
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
        let score = -nega_max(&new_board, depth - 1, eval_fn);
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
