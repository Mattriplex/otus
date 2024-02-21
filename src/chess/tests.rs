use crate::chess::*;

use self::move_checking::{apply_move, get_legal_moves};

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

    assert!(board.can_castle_kingside(White));
    assert!(board.can_castle_queenside(White));
    assert!(board.can_castle_kingside(Black));
    assert!(board.can_castle_queenside(Black));
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

    assert!(!board.can_castle_kingside(White));
    assert!(!board.can_castle_queenside(White));
    assert!(!board.can_castle_kingside(Black));
    assert!(!board.can_castle_queenside(Black));
}

// TODO short game
// TODO stalemate game

// TODO number of reached positions

fn perft(board: &Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut count = 0;
    for mv in get_legal_moves(board) {
        let new_board = apply_move(&board, &mv).unwrap();
        let new_positions = perft(&new_board, depth - 1);
        count += new_positions;
        if depth > 2 { println!("{}: {}", mv, new_positions); }
        
    }
    count
}

#[test]
fn test_perf_starting_pos() {
    let board = Board::default();
    let count = perft(&board, 4);

    assert_eq!(count, 197281);
}

#[test]
fn test_perf_kiwipete() {
    let board =
        Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();

    assert_eq!(perft(&board, 1), 48);
    assert_eq!(perft(&board, 2), 2039);
    assert_eq!(perft(&board, 3), 97862);
}

#[test]
fn test_perf_3() {
    let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();

    assert_eq!(perft(&board, 1), 14);
    assert_eq!(perft(&board, 2), 191);
    assert_eq!(perft(&board, 3), 2812);
}

#[test]
fn test_perf_3_alt() {
    let board = Board::from_fen("8/2p5/3p4/KP5r/1R3pPk/8/4P3/8 b - - 0 1").unwrap();
    let p1 = perft(&board, 2); // 226
    let board2 = Board::from_fen("8/2p5/3p4/KP5r/1R3pPk/8/4P3/8 b - g3 0 1").unwrap();
    //board2 = apply_move(&board2, &Move::Normal{ from: Position(File::G, Rank::_2), to: Position(File::G, Rank::_4)}).unwrap();
    let p2 = perft(&board2, 2); // 224
    // this has to be because of en passant
    assert_eq!(p1, p2);
}

#[test]
fn test_perf_4() {
    let board = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1")
        .unwrap();

    assert_eq!(perft(&board, 1), 6);
    assert_eq!(perft(&board, 2), 264);
    assert_eq!(perft(&board, 3), 9467);
}

#[test]
fn test_perf_5() {
    let board =
        Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8 ").unwrap();

    assert_eq!(perft(&board, 1), 44);
    assert_eq!(perft(&board, 2), 1486);
    assert_eq!(perft(&board, 3), 62379);
}

#[test]
fn test_perf_6() {
    let board =
        Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10")
            .unwrap();

    assert_eq!(perft(&board, 1), 46);
    assert_eq!(perft(&board, 2), 2079);
    assert_eq!(perft(&board, 3), 89890);
}
