use std::result;

use rstest::rstest;

use crate::chess::move_checking::apply_move;
use crate::chess::move_checking::is_move_legal;
use crate::chess::Color::*;
use crate::chess::File::*;
use crate::chess::Opponent;
use crate::chess::PieceType::*;
use crate::chess::Rank::*;
use crate::chess::{Board, Color, File, Move, Piece, PieceType, Position, Rank};

#[rstest]
#[case(White)]
#[case(Black)]
fn test_cannot_move_opponents_piece(#[case] player: Color) {
    let mut board = Board::empty();
    board.active_player = player;
    board.set_piece(E, _2, Piece(PieceType::Queen, player.opponent()));
    let move_ = Move::Normal {
        from: Position(E, _2),
        to: Position(E, _4),
    };

    assert!(!is_move_legal(&board, &move_));
}

#[rstest]
#[case(White)]
#[case(Black)]
fn test_cannot_move_to_same_square(#[case] player: Color) {
    let mut board = Board::empty();
    board.active_player = player;
    board.set_piece(E, _2, Piece(PieceType::Queen, player));
    let move_ = Move::Normal {
        from: Position(E, _2),
        to: Position(E, _2),
    };

    assert!(!is_move_legal(&board, &move_));
}

#[rstest]
#[case(White)]
#[case(Black)]
fn test_cannot_move_from_empty_square(#[case] player: Color) {
    let mut board = Board::empty();
    board.active_player = player;
    let move_ = Move::Normal {
        from: Position(E, _2),
        to: Position(E, _3),
    };

    assert!(!is_move_legal(&board, &move_));
}

#[rstest]
#[case(White)]
#[case(Black)]
fn test_cannot_capture_own_piece(#[case] player: Color) {
    let mut board = Board::empty();
    board.active_player = player;
    board.set_piece(E, _2, Piece(PieceType::Queen, player));
    board.set_piece(E, _3, Piece(PieceType::Queen, player));
    let move_ = Move::Normal {
        from: Position(E, _2),
        to: Position(E, _3),
    };

    assert!(!is_move_legal(&board, &move_));
}

#[rstest]
#[case(Pawn, E, _3)]
#[case(Pawn, E, _4)]
#[case(Knight, C, _1)]
#[case(Knight, C, _3)]
#[case(Knight, D, _4)]
#[case(Knight, F, _4)]
#[case(Knight, G, _3)]
#[case(Knight, G, _1)]
#[case(Bishop, C, _4)]
#[case(Bishop, F, _1)]
#[case(Bishop, A, _6)]
#[case(Bishop, H, _5)]
#[case(Rook, E, _1)]
#[case(Rook, A, _2)]
#[case(Rook, H, _2)]
#[case(Rook, E, _5)]
#[case(Queen, E, _1)]
#[case(Queen, D, _1)]
#[case(Queen, H, _2)]
#[case(King, E, _1)]
#[case(King, F, _3)]
fn test_valid_movement_patterns(
    #[case] piece: PieceType,
    #[case] dst_file: File,
    #[case] dst_rank: Rank,
) {
    let mut board = Board::empty();
    board.active_player = White;
    board.set_piece(E, _2, Piece(piece, White));
    let move_ = Move::Normal {
        from: Position(E, _2),
        to: Position(dst_file, dst_rank),
    };

    assert!(is_move_legal(&board, &move_));
}

#[rstest]
#[case(Pawn, E, _1)]
#[case(Pawn, E, _5)]
#[case(Pawn, D, _3)]
#[case(Pawn, F, _3)]
#[case(Knight, E, _4)]
#[case(Knight, D, _2)]
#[case(Knight, D, _1)]
#[case(Bishop, E, _3)]
#[case(Queen, G, _3)]
#[case(King, E, _4)]
#[case(King, G, _3)]
fn test_invalid_movement_patterns(
    #[case] piece: PieceType,
    #[case] dst_file: File,
    #[case] dst_rank: Rank,
) {
    let mut board = Board::empty();
    board.active_player = White;
    board.set_piece(E, _2, Piece(piece, White));
    let move_ = Move::Normal {
        from: Position(E, _2),
        to: Position(dst_file, dst_rank),
    };

    assert!(!is_move_legal(&board, &move_));
}

#[rstest]
#[case(F, _6, true)]
#[case(F, _5, true)]
#[case(F, _4, false)]
#[case(G, _6, false)]
fn test_black_pawn_moves(#[case] dst_file: File, #[case] dst_rank: Rank, #[case] expected: bool) {
    let mut board = Board::empty();
    board.active_player = Black;
    board.set_piece(F, _7, Piece(Pawn, Black));
    let move_ = Move::Normal {
        from: Position(F, _7),
        to: Position(dst_file, dst_rank),
    };

    assert_eq!(is_move_legal(&board, &move_), expected);
}

#[test]
fn test_blocked_pawn() {
    let mut board = Board::empty();
    board.active_player = Black;
    board.set_piece(F, _7, Piece(Pawn, Black));
    board.set_piece(F, _6, Piece(Pawn, White));
    let move1 = Move::Normal {
        from: Position(F, _7),
        to: Position(F, _6),
    };
    let move2 = Move::Normal {
        from: Position(F, _7),
        to: Position(F, _5),
    };

    assert!(!is_move_legal(&board, &move1));
    assert!(!is_move_legal(&board, &move2));
}

#[rstest]
#[case(White, Pawn, D, _5)]
#[case(White, Pawn, F, _5)]
#[case(Black, Pawn, D, _3)]
#[case(Black, Pawn, F, _3)]
#[case(Black, Knight, F, _2)]
#[case(Black, Bishop, B, _1)]
#[case(White, Rook, H, _4)]
#[case(White, Queen, A, _4)]
#[case(Black, King, F, _5)]
fn test_capture(
    #[case] player: Color,
    #[case] piece: PieceType,
    #[case] dst_file: File,
    #[case] dst_rank: Rank,
) {
    let mut board = Board::empty();
    board.active_player = player;
    board.set_piece(E, _4, Piece(piece, player));
    board.set_piece(
        dst_file,
        dst_rank,
        Piece(PieceType::Pawn, player.opponent()),
    );
    let move_ = Move::Normal {
        from: Position(E, _4),
        to: Position(dst_file, dst_rank),
    };

    let result = apply_move(&board, &move_).unwrap();

    assert_eq!(result.get_piece(E, _4), None);
    assert_eq!(
        result.get_piece(dst_file, dst_rank),
        Some(Piece(piece, player))
    );
}

#[test]
fn test_en_passant_target() {
    let mut board = Board::empty();
    board.active_player = White;
    board.set_piece(A, _2, Piece(PieceType::Pawn, White));
    let move_ = Move::Normal {
        from: Position(A, _2),
        to: Position(A, _4),
    };

    let result = apply_move(&board, &move_).unwrap();

    assert_eq!(result.en_passant_target, Some(Position(A, _3)));
}

#[test]
fn test_en_passant_capture() {
    let mut board = Board::empty();
    board.active_player = White;
    board.en_passant_target = Some(Position(D, _6));
    board.set_piece(D, _5, Piece(PieceType::Pawn, Black));
    board.set_piece(E, _5, Piece(PieceType::Pawn, White));
    let move_ = Move::Normal {
        from: Position(E, _5),
        to: Position(D, _6),
    };

    let result = apply_move(&board, &move_).unwrap();

    assert_eq!(result.get_piece(E, _5), None);
    assert_eq!(result.get_piece(D, _5), None);
    assert_eq!(result.get_piece(D, _6), Some(Piece(PieceType::Pawn, White)));
    assert!(result.en_passant_target.is_none());
}

#[test]
fn test_invalid_en_passant() {
    let mut board = Board::empty();
    board.active_player = White;
    board.en_passant_target = None;
    board.set_piece(D, _5, Piece(PieceType::Pawn, Black));
    board.set_piece(E, _5, Piece(PieceType::Pawn, White));
    let move_ = Move::Normal {
        from: Position(E, _5),
        to: Position(D, _6),
    };

    assert!(!is_move_legal(&board, &move_));
}

#[test]
fn test_no_normal_pawn_move_to_board_end() {
    let mut board = Board::empty();
    board.active_player = Black;
    board.set_piece(F, _2, Piece(PieceType::Pawn, Black));
    let move_ = Move::Normal {
        from: Position(F, _2),
        to: Position(F, _1),
    };

    assert!(!is_move_legal(&board, &move_));
}

#[test]
fn test_promotion_move() {
    let mut board = Board::empty();
    board.active_player = Black;
    board.set_piece(F, _2, Piece(PieceType::Pawn, Black));
    let move_ = Move::Promotion {
        from: Position(F, _2),
        to: Position(F, _1),
        promotion: crate::chess::PromotionPieceType::Queen,
    };

    let result = apply_move(&board, &move_).unwrap();

    assert_eq!(result.get_piece(F, _2), None);
    assert_eq!(
        result.get_piece(F, _1),
        Some(Piece(PieceType::Queen, Black))
    );
}

#[test]
fn test_promotion_capture() {
    let mut board = Board::empty();
    board.active_player = White;
    board.set_piece(B, _7, Piece(PieceType::Pawn, White));
    board.set_piece(A, _8, Piece(PieceType::Rook, Black));
    let move_ = Move::Promotion {
        from: Position(B, _7),
        to: Position(A, _8),
        promotion: crate::chess::PromotionPieceType::Knight,
    };

    let result = apply_move(&board, &move_).unwrap();

    assert_eq!(result.get_piece(B, _7), None);
    assert_eq!(
        result.get_piece(A, _8),
        Some(Piece(PieceType::Knight, White))
    );
}

#[rstest]
#[case(Bishop, D, _4, F, _6)]
#[case(Rook, A, _5, H, _5)]
#[case(Queen, B, _8, H, _2)]
#[case(Queen, E, _7, E, _2)]
fn test_sliding_piece_blocked_by_friendly_piece(
    #[case] piece: PieceType,
    #[case] src_file: File,
    #[case] src_rank: Rank,
    #[case] dst_file: File,
    #[case] dst_rank: Rank,
) {
    let mut board = Board::empty();
    board.active_player = White;
    board.set_piece(src_file, src_rank, Piece(piece, White));
    board.set_piece(E, _5, Piece(PieceType::Pawn, White));
    let move_ = Move::Normal {
        from: Position(src_file, src_rank),
        to: Position(dst_file, dst_rank),
    };

    assert!(!is_move_legal(&board, &move_));
}

#[rstest]
#[case(Bishop, D, _4, F, _6)]
#[case(Rook, A, _5, H, _5)]
#[case(Queen, B, _8, H, _2)]
#[case(Queen, E, _7, E, _2)]
fn test_long_slide_blocked_by_opponent_piece(
    #[case] piece: PieceType,
    #[case] src_file: File,
    #[case] src_rank: Rank,
    #[case] dst_file: File,
    #[case] dst_rank: Rank,
) {
    let mut board = Board::empty();
    board.active_player = Black;
    board.set_piece(src_file, src_rank, Piece(piece, Black));
    board.set_piece(E, _5, Piece(PieceType::Pawn, White));
    let move_ = Move::Normal {
        from: Position(src_file, src_rank),
        to: Position(dst_file, dst_rank),
    };

    assert!(!is_move_legal(&board, &move_));
}

#[test]
fn test_must_not_move_king_into_check() {
    let mut board = Board::empty();
    board.set_piece(E, _1, Piece(PieceType::King, White));
    board.set_piece(F, _8, Piece(PieceType::Rook, Black));
    let move_ = Move::Normal {
        from: Position(E, _1),
        to: Position(F, _2),
    };

    assert!(!is_move_legal(&board, &move_));
}

#[test]
fn test_must_not_leave_king_in_check() {
    let mut board = Board::empty();
    board.set_piece(E, _1, Piece(PieceType::King, White));
    board.set_piece(E, _8, Piece(PieceType::Rook, Black));
    board.set_piece(G, _2, Piece(PieceType::Bishop, White));
    let move_ = Move::Normal {
        from: Position(G, _2),
        to: Position(F, _3),
    };

    assert!(!is_move_legal(&board, &move_));
}

#[test]
fn test_must_not_put_king_in_check() {
    let mut board = Board::empty();
    board.set_piece(E, _1, Piece(PieceType::King, White));
    board.set_piece(E, _8, Piece(PieceType::Rook, Black));
    board.set_piece(E, _4, Piece(PieceType::Bishop, White));
    let move_ = Move::Normal {
        from: Position(E, _4),
        to: Position(F, _3),
    };

    assert!(!is_move_legal(&board, &move_));
}

#[test]
fn test_en_passant_must_not_put_king_in_check() {
    let mut board = Board::empty();
    board.active_player = White;
    board.set_piece(H, _5, Piece(PieceType::King, White));
    board.set_piece(D, _5, Piece(PieceType::Pawn, Black));
    board.set_piece(C, _5, Piece(PieceType::Pawn, White));
    board.set_piece(B, _5, Piece(PieceType::Rook, Black));
    board.en_passant_target = Some(Position(D, _6));
    let move_ = Move::Normal {
        from: Position(C, _5),
        to: Position(D, _6),
    };

    assert!(!is_move_legal(&board, &move_));
}

#[test]
fn test_king_side_castling_must_not_put_king_in_check() {
    let board = Board::from_fen("6r1/8/8/8/8/8/8/4K2R w K - 0 1").unwrap();
    let move_ = Move::CastleKingside;

    assert!(!is_move_legal(&board, &move_));
}

#[test]
fn test_queen_side_castling_must_not_put_king_in_check() {
    let board = Board::from_fen("r3k3/8/8/8/8/8/8/2Q5 b q - 0 1").unwrap();
    let move_ = Move::CastleQueenside;

    assert!(!is_move_legal(&board, &move_));
}

#[test]
fn test_king_side_castling_must_not_move_king_through_check() {
    let board = Board::from_fen("4k2r/8/8/8/8/B7/8/8 b K - 0 1").unwrap();
    let move_ = Move::CastleKingside;

    assert!(!is_move_legal(&board, &move_));
}

#[test]
fn test_queen_side_castling_must_not_move_king_through_check() {
    let board = Board::from_fen("8/8/8/8/8/5b2/8/R3K3 w Q - 0 1").unwrap();
    let move_ = Move::CastleQueenside;

    assert!(!is_move_legal(&board, &move_));
}

#[test]
fn test_cannot_castle_out_of_check() {
    let board = Board::from_fen("4r3/8/8/8/8/8/8/4K2R w K - 0 1").unwrap();
    let move_ = Move::CastleKingside;

    assert!(!is_move_legal(&board, &move_));
}

#[rstest]
#[case(B, Move::CastleQueenside)]
#[case(C, Move::CastleQueenside)]
#[case(D, Move::CastleQueenside)]
#[case(F, Move::CastleKingside)]
#[case(G, Move::CastleKingside)]
fn test_castle_blocked(#[case] file: File, #[case] move_: Move) {
    let mut board = Board::empty();
    board.castling_rights = 0b1111;
    board.set_piece(file, _1, Piece(PieceType::Bishop, White));

    assert!(!is_move_legal(&board, &move_));
}

#[test]
fn test_valid_king_side_castling() {
    let board = Board::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1").unwrap();
    let move_ = Move::CastleKingside;

    let result = apply_move(&board, &move_).unwrap();

    assert_eq!(result.get_piece(E, _1), None);
    assert_eq!(result.get_piece(H, _1), None);
    assert_eq!(result.get_piece(G, _1), Some(Piece(PieceType::King, White)));
    assert_eq!(result.get_piece(F, _1), Some(Piece(PieceType::Rook, White)));
}

#[test]
fn test_valid_queen_side_castling() {
    let board = Board::from_fen("rr6/8/8/8/8/8/8/R3K2R w KQ - 0 1").unwrap();
    let move_ = Move::CastleQueenside;

    let result = apply_move(&board, &move_).unwrap();

    assert_eq!(result.get_piece(E, _1), None);
    assert_eq!(result.get_piece(A, _1), None);
    assert_eq!(result.get_piece(C, _1), Some(Piece(PieceType::King, White)));
    assert_eq!(result.get_piece(D, _1), Some(Piece(PieceType::Rook, White)));
}

#[test]
fn test_missing_castling_rights() {
    let board = Board::from_fen("8/8/8/8/8/8/8/R3K2R w - - 0 1").unwrap();
    let move_ = Move::CastleKingside;

    assert!(!is_move_legal(&board, &move_));
}

#[test]
fn test_king_move_voids_white_castling_rights() {
    let board = Board::from_fen("8/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
    let move_ = Move::Normal {
        from: Position(E, _1),
        to: Position(E, _2),
    };

    let result = apply_move(&board, &move_).unwrap();

    assert!(
        !(result.can_castle_kingside(Color::White) || result.can_castle_queenside(Color::White))
    );
}

#[test]
fn test_king_move_voids_black_castling_rights() {
    let board = Board::from_fen("r3k2r/8/8/8/8/8/8/8 b KQkq - 0 1").unwrap();
    let move_ = Move::Normal {
        from: Position(E, _8),
        to: Position(E, _7),
    };

    let result = apply_move(&board, &move_).unwrap();

    assert!(
        !(result.can_castle_kingside(Color::Black) || result.can_castle_queenside(Color::Black))
    );
}

#[test]
fn test_rook_move_voids_castling_rights() {
    let board = Board::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1").unwrap();
    let move_ = Move::Normal {
        from: Position(H, _1),
        to: Position(H, _2),
    };

    let result = apply_move(&board, &move_).unwrap();

    assert!(result.can_castle_queenside(Color::White));
    assert!(!result.can_castle_kingside(Color::White));
}

#[test]
fn test_rook_capture_voids_castling_rights() {
    let board = Board::from_fen("7b/8/8/8/8/8/8/R3K2R b KQ - 0 1").unwrap();
    let move_ = Move::Normal {
        from: Position(H, _8),
        to: Position(A, _1),
    };

    let result = apply_move(&board, &move_).unwrap();

    assert!(!result.can_castle_queenside(Color::White));
    assert!(result.can_castle_kingside(Color::White));
}

// TODO pawn check
