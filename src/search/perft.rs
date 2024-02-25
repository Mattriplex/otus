use crate::board::{move_checking::apply_legal_move, Board};

pub fn perft(board: &Board, depth: i8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut count = 0;
    for mv in board.get_legal_moves() {
        let new_board = apply_legal_move(board, &mv);
        let new_positions = perft_rec(&new_board, depth - 1);
        println!("{}: {}", mv.to_move(board), new_positions);
        count += new_positions;
    }

    println!("Nodes searched: {}", count);
    count
}

fn perft_rec(board: &Board, depth: i8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut count = 0;
    for mv in board.get_legal_moves() {
        let new_board = apply_legal_move(board, &mv);
        let new_positions = perft_rec(&new_board, depth - 1);
        count += new_positions;
    }
    count
}
