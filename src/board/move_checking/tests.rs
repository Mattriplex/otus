use rstest::rstest;

use crate::board::model_utils::ColorProps;
use crate::board::move_checking::apply_move;
use crate::board::move_checking::is_move_legal;
use crate::board::Color::*;
use crate::board::File::*;
use crate::board::PieceType::*;
use crate::board::Rank::*;
use crate::board::{Board, Color, File, Move, Piece, PieceType, Rank, Square};

const A2: Square = Square(A, _2);
const A4: Square = Square(A, _4);
const A8: Square = Square(A, _8);
const B7: Square = Square(B, _7);
const C5: Square = Square(C, _5);
const D5: Square = Square(D, _5);
const D6: Square = Square(D, _6);
const E1: Square = Square(E, _1);
const E2: Square = Square(E, _2);
const E3: Square = Square(E, _3);
const E4: Square = Square(E, _4);
const E5: Square = Square(E, _5);
const F1: Square = Square(F, _1);
const F2: Square = Square(F, _2);
const F3: Square = Square(F, _3);
const F6: Square = Square(F, _6);
const F7: Square = Square(F, _7);
const G2: Square = Square(G, _2);

#[rstest]
#[case(White)]
#[case(Black)]
fn test_cannot_move_opponents_piece(#[case] player: Color) {
    let mut board = Board::from_fen("k7/8/8/8/8/8/8/K7 b - - 0 1").unwrap();
    board.active_player = player;
    board.set_piece_at(E2, Piece(PieceType::Queen, player.opponent()));
    let move_ = Move::Normal { src: E2, dest: E4 };

    assert!(!is_move_legal(&board, &move_));
}

#[rstest]
#[case(White)]
#[case(Black)]
fn test_cannot_move_to_same_square(#[case] player: Color) {
    let mut board = Board::from_fen("k7/8/8/8/8/8/8/K7 b - - 0 1").unwrap();
    board.active_player = player;
    board.set_piece_at(E2, Piece(PieceType::Queen, player));
    let move_ = Move::Normal { src: E2, dest: E2 };

    assert!(!is_move_legal(&board, &move_));
}

#[rstest]
#[case(White)]
#[case(Black)]
fn test_cannot_move_from_empty_square(#[case] player: Color) {
    let mut board = Board::from_fen("k7/8/8/8/8/8/8/K7 b - - 0 1").unwrap();
    board.active_player = player;
    let move_ = Move::Normal { src: E2, dest: E3 };

    assert!(!is_move_legal(&board, &move_));
}

#[rstest]
#[case(White)]
#[case(Black)]
fn test_cannot_capture_own_piece(#[case] player: Color) {
    let mut board = Board::from_fen("k7/8/8/8/8/8/8/K7 b - - 0 1").unwrap();
    board.active_player = player;
    board.set_piece_at(E2, Piece(PieceType::Queen, player));
    board.set_piece_at(E3, Piece(PieceType::Queen, player));
    let move_ = Move::Normal { src: E2, dest: E3 };

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
    let mut board = Board::from_fen("8/8/8/8/8/8/8/K7 w - - 0 1").unwrap();
    board.active_player = White;
    board.set_piece_at(E2, Piece(piece, White));
    let move_ = Move::Normal {
        src: E2,
        dest: Square(dst_file, dst_rank),
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
    let mut board = Board::from_fen("k7/8/8/8/8/8/8/K7 b - - 0 1").unwrap();
    board.active_player = White;
    board.set_piece_at(E2, Piece(piece, White));
    let move_ = Move::Normal {
        src: E2,
        dest: Square(dst_file, dst_rank),
    };

    assert!(!is_move_legal(&board, &move_));
}

#[rstest]
#[case(F, _6, true)]
#[case(F, _5, true)]
#[case(F, _4, false)]
#[case(G, _6, false)]
fn test_black_pawn_moves(#[case] dst_file: File, #[case] dst_rank: Rank, #[case] expected: bool) {
    let mut board = Board::from_fen("k7/8/8/8/8/8/8/K7 b - - 0 1").unwrap();
    board.active_player = Black;
    board.set_piece_at(F7, Piece(Pawn, Black));
    let move_ = Move::Normal {
        src: Square(F, _7),
        dest: Square(dst_file, dst_rank),
    };

    assert_eq!(is_move_legal(&board, &move_), expected);
}

#[test]
fn test_blocked_pawn() {
    let mut board = Board::from_fen("k7/8/8/8/8/8/8/K7 b - - 0 1").unwrap();
    board.active_player = Black;
    board.set_piece_at(F7, Piece(Pawn, Black));
    board.set_piece_at(F6, Piece(Pawn, White));
    let move1 = Move::Normal {
        src: Square(F, _7),
        dest: Square(F, _6),
    };
    let move2 = Move::Normal {
        src: Square(F, _7),
        dest: Square(F, _5),
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
    let mut board = Board::from_fen("k7/8/8/8/8/8/8/K7 b - - 0 1").unwrap();
    board.active_player = player;
    board.set_piece_at(E4, Piece(piece, player));
    board.set_piece_at(
        Square(dst_file, dst_rank),
        Piece(PieceType::Pawn, player.opponent()),
    );
    let move_ = Move::Normal {
        src: E4,
        dest: Square(dst_file, dst_rank),
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
    let mut board = Board::from_fen("k7/8/8/8/8/8/8/K7 b - - 0 1").unwrap();
    board.active_player = White;
    board.set_piece_at(A2, Piece(PieceType::Pawn, White));
    let move_ = Move::Normal { src: A2, dest: A4 };

    let result = apply_move(&board, &move_).unwrap();

    assert_eq!(result.en_passant_target, Some(Square(A, _3)));
}

#[test]
fn test_en_passant_capture() {
    let mut board = Board::from_fen("k7/8/8/8/8/8/8/K7 b - - 0 1").unwrap();
    board.active_player = White;
    board.en_passant_target = Some(Square(D, _6));
    board.set_piece_at(D5, Piece(PieceType::Pawn, Black));
    board.set_piece_at(E5, Piece(PieceType::Pawn, White));
    let move_ = Move::Normal {
        src: Square(E, _5),
        dest: Square(D, _6),
    };

    let result = apply_move(&board, &move_).unwrap();

    assert_eq!(result.get_piece(E, _5), None);
    assert_eq!(result.get_piece(D, _5), None);
    assert_eq!(result.get_piece(D, _6), Some(Piece(PieceType::Pawn, White)));
    assert!(result.en_passant_target.is_none());
}

#[test]
fn test_invalid_en_passant() {
    let mut board = Board::from_fen("k7/8/8/8/8/8/8/K7 b - - 0 1").unwrap();
    board.active_player = White;
    board.en_passant_target = None;
    board.set_piece_at(D5, Piece(PieceType::Pawn, Black));
    board.set_piece_at(E5, Piece(PieceType::Pawn, White));
    let move_ = Move::Normal { src: E5, dest: D6 };

    assert!(!is_move_legal(&board, &move_));
}

#[test]
fn test_no_normal_pawn_move_to_board_end() {
    let board = Board::from_fen("k7/8/8/8/8/8/5p2/8 b - - 0 1").unwrap();
    let move_ = Move::Normal {
        src: Square(F, _2),
        dest: Square(F, _1),
    };

    assert!(!is_move_legal(&board, &move_));
}

#[test]
fn test_promotion_move() {
    let board = Board::from_fen("k7/8/8/8/8/8/5p2/8 b - - 0 1").unwrap();
    let move_ = Move::Promotion {
        src: F2,
        dest: F1,
        promotion: crate::board::PromotionPieceType::Queen,
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
    let board = Board::from_fen("r7/1P6/8/8/8/8/8/K7 w - - 0 1").unwrap();
    let move_ = Move::Promotion {
        src: B7,
        dest: A8,
        promotion: crate::board::PromotionPieceType::Knight,
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
    let mut board = Board::from_fen("8/8/8/4P3/8/8/8/K7 w - - 0 1").unwrap();
    board.active_player = White;
    board.set_piece_at(Square(src_file, src_rank), Piece(piece, White));
    let move_ = Move::Normal {
        src: Square(src_file, src_rank),
        dest: Square(dst_file, dst_rank),
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
    let mut board = Board::from_fen("k7/8/8/4P3/8/8/8/8 b - - 0 1").unwrap();
    board.set_piece_at(Square(src_file, src_rank), Piece(piece, Black));
    let move_ = Move::Normal {
        src: Square(src_file, src_rank),
        dest: Square(dst_file, dst_rank),
    };

    assert!(!is_move_legal(&board, &move_));
}

#[test]
fn test_must_not_move_king_into_check() {
    let board = Board::from_fen("5r2/8/8/8/8/8/8/4K3 w - - 0 1").unwrap();
    let move_ = Move::Normal { src: E1, dest: F2 };

    assert!(!is_move_legal(&board, &move_));
}

#[test]
fn test_must_not_leave_king_in_check() {
    let board = Board::from_fen("4r3/8/8/8/8/8/6B1/4K3 w - - 0 1").unwrap();
    let move_ = Move::Normal { src: G2, dest: F3 };

    assert!(!is_move_legal(&board, &move_));
}

#[test]
fn test_must_not_put_king_in_check() {
    let board = Board::from_fen("4r3/8/8/8/4B3/8/8/4K3 w - - 0 1").unwrap();
    let move_ = Move::Normal { src: E4, dest: F3 };

    assert!(!is_move_legal(&board, &move_));
}

#[test]
fn test_en_passant_must_not_put_king_in_check() {
    let board = Board::from_fen("8/8/8/r1Pp3K/8/8/8/8 w - d6 0 1").unwrap();
    let move_ = Move::Normal { src: C5, dest: D6 };

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
    let mut board = Board::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1").unwrap();
    board.set_piece_at(Square(file, _1), Piece(PieceType::Bishop, White));

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
        src: Square(E, _1),
        dest: E2,
    };

    let result = apply_move(&board, &move_).unwrap();

    assert!(
        !(result.has_kingside_castling_rights(Color::White)
            || result.has_queenside_castling_rights(Color::White))
    );
}

#[test]
fn test_king_move_voids_black_castling_rights() {
    let board = Board::from_fen("r3k2r/8/8/8/8/8/8/8 b KQkq - 0 1").unwrap();
    let move_ = Move::Normal {
        src: Square(E, _8),
        dest: Square(E, _7),
    };

    let result = apply_move(&board, &move_).unwrap();

    assert!(
        !(result.has_kingside_castling_rights(Color::Black)
            || result.has_queenside_castling_rights(Color::Black))
    );
}

#[test]
fn test_rook_move_voids_castling_rights() {
    let board = Board::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1").unwrap();
    let move_ = Move::Normal {
        src: Square(H, _1),
        dest: Square(H, _2),
    };

    let result = apply_move(&board, &move_).unwrap();

    assert!(result.has_queenside_castling_rights(Color::White));
    assert!(!result.has_kingside_castling_rights(Color::White));
}

#[test]
fn test_rook_capture_voids_castling_rights() {
    let board = Board::from_fen("6kb/8/8/8/8/8/8/R3K2R b KQ - 0 1").unwrap();
    let move_ = Move::Normal {
        src: Square(H, _8),
        dest: Square(A, _1),
    };

    let result = apply_move(&board, &move_).unwrap();

    assert!(!result.has_queenside_castling_rights(Color::White));
    assert!(result.has_kingside_castling_rights(Color::White));
}

// TODO pawn check
// Castling revoked by castling
// Check blocked by enemy piece
