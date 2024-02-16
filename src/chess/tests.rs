#[cfg(test)]
mod tests {
    use crate::chess::*;

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
}
