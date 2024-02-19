    use crate::chess::move_checking::is_move_legal;
    use crate::chess::Color::*;
    use crate::chess::File::*;
    use crate::chess::PieceType::*;
    use crate::chess::Rank::*;
    use crate::chess::{Board, Color, File, Move, Piece, PieceType, Position, Rank};


    #[test]
    fn test_cannot_move_opponents_piece() {}

    #[test]
    fn test_cannot_move_to_same_square() {}

    #[test]
    fn test_cannot_capture_own_piece() {}

    #[test]
    fn test_valid_movement_patterns() {} //paramtereize piece, start, end

    #[test]
    fn test_invalid_movement_patterns() {}

    #[test]
    fn test_double_pawn_move() {}

    #[test]
    fn test_invalid_double_pawn_move() {}

    #[test]
    fn test_blocked_pawn() {}

    #[test]
    fn test_capture() {}

    #[test]
    fn test_en_passant() {}

    #[test]
    fn test_invalid_en_passant() {}

    #[test]
    fn test_no_normal_pawn_move_to_board_end() {}

    #[test]
    fn test_promotion_move() {}

    #[test]
    fn test_promotion_capture() {}

    #[test]
    fn test_sliding_piece_blocked_by_friendly_piece() {}

    #[test]
    fn test_must_not_move_king_into_check() {}

    #[test]
    fn test_must_not_leave_king_in_check() {}

    #[test]
    fn test_must_not_put_king_in_check() {}

    #[test]
    fn test_en_passant_must_not_put_king_in_check() {}

    #[test]
    fn test_castling_must_not_put_king_in_check() {}

    #[test]
    fn test_castling_must_not_move_king_through_check() {}

    #[test]
    fn test_valid_castling() {}

    #[test]
    fn test_missing_castling_rights() {}

    #[test]
    fn test_king_move_voids_castling_rights() {}

    #[test]
    fn test_rook_move_voids_castling_rights() {}

    #[test]
    fn test_rook_capture_voids_castling_rights() {} 


    