use std::sync::mpsc;

use crate::{
    board::{
        models::{GameState, LegalMove},
        move_generation::{apply_legal_move, is_king_in_check},
        Board,
    },
    hashing::{get_zobrist_hash, update_zobrist_hash, TranspositionEntry, TranspositionTable}, search::minimax::get_noise,
};

use super::SearchResult;



/**
 * Never call this in a checkmated or stalemate position, i.e. positions without legal moves. Use Board::getGameState to ensure the precondition if necessary.
 * Listens for termination signal each time a move has been evaluated completely.
 * Prints UCI-conformant search updates to stdout
 */
pub fn search_minimax_threaded_cached(
    board: &Board,
    depth: u8,
    eval_fn: fn(&Board) -> f32,
    trans_table: &mut TranspositionTable,
    rx: mpsc::Receiver<()>,
) {
    let moves = board.get_legal_moves();
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

fn get_cached_eval_or_inf(board: &Board, board_hash: u64, move_: &LegalMove, cache: &TranspositionTable) -> f32 {
    let hash = update_zobrist_hash(board, board_hash, move_);
    match cache.get(hash) {
        Some(entry) => entry.value,
        None => f32::MAX,
    }
}

#[inline]
fn get_leaf_eval(board: &Board, board_hash: u64, eval_fn: fn(&Board) -> f32, trans_table: &mut TranspositionTable,
) -> SearchResult {
    let eval = match board.get_gamestate() {
        GameState::Mated(_) => f32::MIN,
        GameState::Stalemate => 0.0,
        GameState::InProgress => eval_fn(board),
    };
    trans_table.put(
        board_hash,
        TranspositionEntry {
            depth: 0,
            value: eval,
        },
    ); // TODO experiment if this is actually faster
    SearchResult {
        eval,
        nodes_searched: 1,
    }
}

#[inline]
fn get_cached_eval(trans_table: &mut TranspositionTable, board_hash: u64, depth: u8) -> Option<SearchResult> {
    let cache_entry = trans_table.get(board_hash);
    if let Some(entry) = cache_entry {
        if entry.depth >= depth {
            return Some(SearchResult {
                eval: entry.value,
                nodes_searched: 0,
            });
        }
    }
    None
}

fn nega_max_cached(
    board: &Board,
    depth: u8,
    mut alpha: f32,
    beta: f32,
    eval_fn: fn(&Board) -> f32,
    trans_table: &mut TranspositionTable,
    board_hash: u64,
) -> SearchResult {
    if let Some(cached_result) = get_cached_eval(trans_table, board_hash, depth) {
        return cached_result
    }
    if depth == 0 {
        return get_leaf_eval(board, board_hash, eval_fn, trans_table);
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
            TranspositionEntry {
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
        let eval_a = get_cached_eval_or_inf(board, board_hash, a, trans_table);
        let eval_b = get_cached_eval_or_inf(board, board_hash, b, trans_table);
        eval_a.total_cmp(&eval_b) // want to sort valuations in ascending order, these are opponent evals, opps worst situation is my best move
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
        TranspositionEntry {
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

/** Used as an entrypoint for the benchmark */
pub fn search_minimax_cached(
    board: &Board,
    depth: u8,
    eval_fn: fn(&Board) -> f32,
    trans_table: &mut TranspositionTable,
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