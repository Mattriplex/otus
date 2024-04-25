use crate::board::{Board};

pub fn perft(board: &mut Board, depth: i8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut count = 0;
    for mv in board.get_legal_moves() {
        board.make_move(&mv);
        let new_positions = perft_rec(board, depth - 1);
        println!("{}: {}", mv.to_move(board), new_positions);
        count += new_positions;
        board.unmake_move(&mv);
    }

    println!("Nodes searched: {}", count);
    count
}

fn perft_rec(board: &mut Board, depth: i8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut count = 0;
    for mv in board.get_legal_moves() {
        board.make_move(&mv);
        let new_positions = perft_rec(board, depth - 1);
        count += new_positions;
        board.unmake_move(&mv)
    }
    count
}
