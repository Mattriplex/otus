use rand::Rng;

use crate::board::{models::Move, move_checking::apply_move, Board};

use super::eval::get_material_eval;

fn get_noise() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(-0.1..0.1)
}

pub fn search_minimax(board: &Board, depth: u32) -> Move {
    let moves = board.get_legal_moves();
    let mut best_move = moves[0].clone();
    let mut best_score = f32::MIN;
    for move_ in moves {
        let new_board = apply_move(board, &move_).unwrap();
        let score = -nega_max(&new_board, depth - 1);
        if score > best_score {
            best_score = score;
            best_move = move_;
        }
    }
    best_move
}

fn nega_max(board: &Board, depth: u32) -> f32 {
    if depth == 0 {
        return get_material_eval(board);
    }
    let moves = board.get_legal_moves();
    let mut best_score = f32::MIN;
    for move_ in moves {
        let new_board = apply_move(board, &move_).unwrap();
        let score = -nega_max(&new_board, depth - 1) + get_noise(); // add noise to shuffle moves of equal value
        if score > best_score {
            best_score = score;
        }
    }
    best_score
}