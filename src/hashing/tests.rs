use crate::board::{
    models::{File, LegalMove, Square}, move_checking::apply_legal_move, Board
};

use super::{get_zobrist_hash, update_zobrist_hash};

#[test]
pub fn test_transposition_hashes_match() {
    let board: Board = Board::default();
    let init_hash = get_zobrist_hash(&board);
    let (pawn_move_w, knight_move_w, pawn_move_b, knight_move_b) = (
        LegalMove::Normal {
            src: Square::from_string("e2").unwrap(),
            dest: Square::from_string("e3").unwrap(),
            castle_mask: 0,
            captured_piece: None,
        },
        LegalMove::Normal {
            src: Square::from_string("b1").unwrap(),
            dest: Square::from_string("c3").unwrap(),
            castle_mask: 0,
            captured_piece: None,
        },
        LegalMove::Normal {
            src: Square::from_string("e7").unwrap(),
            dest: Square::from_string("e6").unwrap(),
            castle_mask: 0,
            captured_piece: None,
        },
        LegalMove::Normal {
            src: Square::from_string("b8").unwrap(),
            dest: Square::from_string("c6").unwrap(),
            castle_mask: 0,
            captured_piece: None,
        },
    );
    let (board_1, hash_1) = [pawn_move_w.clone(), pawn_move_b.clone(), knight_move_w.clone(), knight_move_b.clone()]
    .iter()
    .fold((board, init_hash), |(board, hash), mv| {
        let new_hash = update_zobrist_hash(&board, hash, mv.clone());
        let new_board = apply_legal_move(&board, mv);
        (new_board, new_hash)
    });
    let (board_2, hash_2) = [knight_move_w, knight_move_b, pawn_move_w, pawn_move_b]
    .iter()
    .fold((board, init_hash), |(board, hash), mv| {
        let new_hash = update_zobrist_hash(&board, hash, mv.clone());
        let new_board = apply_legal_move(&board, mv);
        (new_board, new_hash)
    });

    assert_eq!(hash_1, get_zobrist_hash(&board_1));
    assert_eq!(hash_2, get_zobrist_hash(&board_2));
    assert_eq!(hash_1, hash_2);
}

// TODO larger testcase found through real search
