use rstest::rstest;
use tests::move_checking::get_legal_move_from_move;

use crate::{board::*, search::perft::{self, perft}};

use self::move_checking::apply_legal_move;

#[test]
fn test_fen_default_board() {
    use Color::*;
    use File::*;
    use PieceType::*;
    use Rank::*;

    let board: Board =
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    assert_eq!(board.get_piece(A, _1), Some(Piece(Rook, White)));
    assert_eq!(board.get_piece(A, _2), Some(Piece(Pawn, White)));
    assert!(board.get_piece(A, _3).is_none());
    assert!(board.get_piece(A, _4).is_none());
    assert!(board.get_piece(A, _5).is_none());
    assert!(board.get_piece(A, _6).is_none());
    assert_eq!(board.get_piece(A, _7), Some(Piece(Pawn, Black)));
    assert_eq!(board.get_piece(A, _8), Some(Piece(Rook, Black)));

    assert_eq!(board.get_piece(B, _1), Some(Piece(Knight, White)));
    assert_eq!(board.get_piece(B, _2), Some(Piece(Pawn, White)));
    assert!(board.get_piece(B, _3).is_none());
    assert!(board.get_piece(B, _4).is_none());
    assert!(board.get_piece(B, _5).is_none());
    assert!(board.get_piece(B, _6).is_none());
    assert_eq!(board.get_piece(B, _7), Some(Piece(Pawn, Black)));
    assert_eq!(board.get_piece(B, _8), Some(Piece(Knight, Black)));

    assert_eq!(board.get_piece(C, _1), Some(Piece(Bishop, White)));
    assert_eq!(board.get_piece(C, _2), Some(Piece(Pawn, White)));
    assert!(board.get_piece(C, _3).is_none());
    assert!(board.get_piece(C, _4).is_none());
    assert!(board.get_piece(C, _5).is_none());
    assert!(board.get_piece(C, _6).is_none());
    assert_eq!(board.get_piece(C, _7), Some(Piece(Pawn, Black)));
    assert_eq!(board.get_piece(C, _8), Some(Piece(Bishop, Black)));

    assert_eq!(board.get_piece(D, _1), Some(Piece(Queen, White)));
    assert_eq!(board.get_piece(D, _2), Some(Piece(Pawn, White)));
    assert!(board.get_piece(D, _3).is_none());
    assert!(board.get_piece(D, _4).is_none());
    assert!(board.get_piece(D, _5).is_none());
    assert!(board.get_piece(D, _6).is_none());
    assert_eq!(board.get_piece(D, _7), Some(Piece(Pawn, Black)));
    assert_eq!(board.get_piece(D, _8), Some(Piece(Queen, Black)));

    assert_eq!(board.get_piece(E, _1), Some(Piece(King, White)));
    assert_eq!(board.get_piece(E, _2), Some(Piece(Pawn, White)));
    assert!(board.get_piece(E, _3).is_none());
    assert!(board.get_piece(E, _4).is_none());
    assert!(board.get_piece(E, _5).is_none());
    assert!(board.get_piece(E, _6).is_none());
    assert_eq!(board.get_piece(E, _7), Some(Piece(Pawn, Black)));
    assert_eq!(board.get_piece(E, _8), Some(Piece(King, Black)));

    assert_eq!(board.get_piece(F, _1), Some(Piece(Bishop, White)));
    assert_eq!(board.get_piece(F, _2), Some(Piece(Pawn, White)));
    assert!(board.get_piece(F, _3).is_none());
    assert!(board.get_piece(F, _4).is_none());
    assert!(board.get_piece(F, _5).is_none());
    assert!(board.get_piece(F, _6).is_none());
    assert_eq!(board.get_piece(F, _7), Some(Piece(Pawn, Black)));
    assert_eq!(board.get_piece(F, _8), Some(Piece(Bishop, Black)));

    assert_eq!(board.get_piece(G, _1), Some(Piece(Knight, White)));
    assert_eq!(board.get_piece(G, _2), Some(Piece(Pawn, White)));
    assert!(board.get_piece(G, _3).is_none());
    assert!(board.get_piece(G, _4).is_none());
    assert!(board.get_piece(G, _5).is_none());
    assert!(board.get_piece(G, _6).is_none());
    assert_eq!(board.get_piece(G, _7), Some(Piece(Pawn, Black)));
    assert_eq!(board.get_piece(G, _8), Some(Piece(Knight, Black)));

    assert_eq!(board.get_piece(H, _1), Some(Piece(Rook, White)));
    assert_eq!(board.get_piece(H, _2), Some(Piece(Pawn, White)));
    assert!(board.get_piece(H, _3).is_none());
    assert!(board.get_piece(H, _4).is_none());
    assert!(board.get_piece(H, _5).is_none());
    assert!(board.get_piece(H, _6).is_none());
    assert_eq!(board.get_piece(H, _7), Some(Piece(Pawn, Black)));
    assert_eq!(board.get_piece(H, _8), Some(Piece(Rook, Black)));

    assert_eq!(board.active_player, White);

    assert!(board.has_kingside_castling_rights(White));
    assert!(board.has_queenside_castling_rights(White));
    assert!(board.has_kingside_castling_rights(Black));
    assert!(board.has_queenside_castling_rights(Black));
}

#[test]
fn test_fen_board() {
    use Color::*;
    use File::*;
    use PieceType::*;
    use Rank::*;

    let board: Board =
        Board::from_fen("r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 b - e3 0 1").unwrap();

    assert_eq!(board.get_piece(A, _1), Some(Piece(Queen, Black)));
    assert_eq!(board.get_piece(A, _2), Some(Piece(Pawn, White)));
    assert!(board.get_piece(A, _3).is_none());
    assert!(board.get_piece(A, _4).is_none());
    assert!(board.get_piece(A, _5).is_none());
    assert_eq!(board.get_piece(A, _6), Some(Piece(Knight, Black)));
    assert_eq!(board.get_piece(A, _7), Some(Piece(Pawn, Black)));
    assert_eq!(board.get_piece(A, _8), Some(Piece(Rook, Black)));

    assert!(board.get_piece(B, _1).is_none());
    assert!(board.get_piece(B, _2).is_none());
    assert!(board.get_piece(B, _3).is_none());
    assert!(board.get_piece(B, _4).is_none());
    assert_eq!(board.get_piece(B, _5), Some(Piece(Pawn, Black)));
    assert!(board.get_piece(B, _6).is_none());
    assert!(board.get_piece(B, _7).is_none());
    assert!(board.get_piece(B, _8).is_none());

    assert!(board.get_piece(C, _1).is_none());
    assert_eq!(board.get_piece(C, _2), Some(Piece(Pawn, White)));
    assert!(board.get_piece(C, _3).is_none());
    assert!(board.get_piece(C, _4).is_none());
    assert!(board.get_piece(C, _5).is_none());
    assert!(board.get_piece(C, _6).is_none());
    assert!(board.get_piece(C, _7).is_none());
    assert_eq!(board.get_piece(C, _8), Some(Piece(Bishop, Black)));

    assert!(board.get_piece(D, _1).is_none());
    assert!(board.get_piece(D, _2).is_none());
    assert_eq!(board.get_piece(D, _3), Some(Piece(Pawn, White)));
    assert!(board.get_piece(D, _4).is_none());
    assert_eq!(board.get_piece(D, _5), Some(Piece(Knight, White)));
    assert!(board.get_piece(D, _6).is_none());
    assert_eq!(board.get_piece(D, _7), Some(Piece(Pawn, Black)));
    assert_eq!(board.get_piece(D, _8), Some(Piece(King, Black)));

    assert!(board.get_piece(E, _1).is_none());
    assert_eq!(board.get_piece(E, _2), Some(Piece(King, White)));
    assert!(board.get_piece(E, _3).is_none());
    assert!(board.get_piece(E, _4).is_none());
    assert_eq!(board.get_piece(E, _5), Some(Piece(Pawn, White)));
    assert!(board.get_piece(E, _6).is_none());
    assert_eq!(board.get_piece(E, _7), Some(Piece(Bishop, White)));
    assert!(board.get_piece(E, _8).is_none());

    assert!(board.get_piece(F, _1).is_none());
    assert!(board.get_piece(F, _2).is_none());
    assert!(board.get_piece(F, _3).is_none());
    assert!(board.get_piece(F, _4).is_none());
    assert!(board.get_piece(F, _5).is_none());
    assert_eq!(board.get_piece(F, _6), Some(Piece(Knight, Black)));
    assert_eq!(board.get_piece(F, _7), Some(Piece(Pawn, Black)));
    assert!(board.get_piece(F, _8).is_none());

    assert_eq!(board.get_piece(G, _1), Some(Piece(Bishop, Black)));
    assert!(board.get_piece(G, _2).is_none());
    assert!(board.get_piece(G, _3).is_none());
    assert_eq!(board.get_piece(G, _4), Some(Piece(Pawn, White)));
    assert!(board.get_piece(G, _5).is_none());
    assert!(board.get_piece(G, _6).is_none());
    assert_eq!(board.get_piece(G, _7), Some(Piece(Knight, White)));
    assert!(board.get_piece(G, _8).is_none());

    assert!(board.get_piece(H, _1).is_none());
    assert!(board.get_piece(H, _2).is_none());
    assert!(board.get_piece(H, _3).is_none());
    assert!(board.get_piece(H, _4).is_none());
    assert_eq!(board.get_piece(H, _5), Some(Piece(Pawn, White)));
    assert!(board.get_piece(H, _6).is_none());
    assert_eq!(board.get_piece(H, _7), Some(Piece(Pawn, Black)));
    assert_eq!(board.get_piece(H, _8), Some(Piece(Rook, Black)));

    assert_eq!(board.active_player, Black);

    assert!(!board.has_kingside_castling_rights(White));
    assert!(!board.has_queenside_castling_rights(White));
    assert!(!board.has_kingside_castling_rights(Black));
    assert!(!board.has_queenside_castling_rights(Black));
}

/*
fn perft(board: &Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut count = 0;
    for mv in board.get_legal_moves() {
        let new_board = apply_legal_move(board, &mv);
        let new_positions = perft(&new_board, depth - 1);
        count += new_positions;
        //if depth > 2 { println!("{}: {}", mv, new_positions); }
    }
    count
}
 */

#[test]
fn test_perf_starting_pos() {
    let mut board = Board::default();
    let count = perft(&mut board, 4);

    assert_eq!(count, 197281);
}

#[test]
fn test_perf_kiwipete() {
    let mut board =
        Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();

    assert_eq!(perft(&mut board, 3), 97862);
}

#[test]
fn test_perf_3() {
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();

    assert_eq!(perft(&mut board, 1), 14);
    assert_eq!(perft(&mut board, 2), 191);
    assert_eq!(perft(&mut board, 3), 2812);
    assert_eq!(perft(&mut board, 4), 43238);
}

#[test]
fn test_perf_4() {
    let mut board =
        Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1")
            .unwrap();

    assert_eq!(perft(&mut board, 1), 6);
    assert_eq!(perft(&mut board, 2), 264);
    assert_eq!(perft(&mut board, 3), 9467);
}

#[test]
fn test_perf_5() {
    let mut board =
        Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8 ").unwrap();

    assert_eq!(perft(&mut board, 3), 62379);
}

#[test]
fn test_perf_6() {
    let mut board =
        Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10")
            .unwrap();

    assert_eq!(perft(&mut board, 3), 89890);
}

#[test]
fn test_fen_output() {
    let board = Board::default();
    let fen = board.to_fen();
    assert_eq!(
        fen,
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    );
}

#[test]
fn test_illegal_pawn_move() {
    let board = Board::from_fen("8/2p5/3p4/KP5r/1R2Pp1k/8/6P1/8 b - e3 0 1").unwrap();

    assert!(!is_move_legal(
        &board,
        &Move::from_uci_string(&board, "c7b6").unwrap()
    ));
}

#[test]
fn test_queen_promotion() {
    let board = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P1RPP/R2Q2K1 b kq - 1 1")
        .unwrap();
    let mv =
        get_legal_move_from_move(&board, &Move::from_uci_string(&board, "b2a1q").unwrap()).unwrap();

    assert!(board.get_legal_moves().contains(&mv));
}

#[rstest]
#[case("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 2)]
pub fn test_revert_perft(#[case] fen: &str, #[case] depth: i8) {
    let mut board = Board::from_fen(fen).unwrap();
    let initial_fen = board.to_fen();
    for mv in board.get_legal_moves() {
        let en_passant_target = board.en_passant_target;
        board.make_move(&mv);
        perft_rec(&mut board, depth);
        board.unmake_move(&mv);
        board.en_passant_target = en_passant_target; // TODO make unmake move do this
        assert_eq!(board.to_fen(), initial_fen);
    }
}

fn perft_rec(board: &mut Board, depth: i8) -> u64 {
    if depth == 0 {
        return 1;
    }
    let initial_fen = board.to_fen();
    let mut count = 0;
    for mv in board.get_legal_moves() {
        let en_passant_target = board.en_passant_target;
        board.make_move(&mv);
        let new_positions = perft_rec(board, depth - 1);
        count += new_positions;
        board.unmake_move(&mv);
        board.en_passant_target = en_passant_target;
        assert_eq!(board.to_fen(), initial_fen);
    }
    count
}


#[test]
pub fn test_castling_right_bits() {
    let mut board = Board::from_fen("r1b1k3/ppBpnN1r/2n1p3/6pp/1b1P2P1/P1NR3P/1PP2P2/2K2B1R b q - 2 16").unwrap();
    perft(&mut board, 3);
}
