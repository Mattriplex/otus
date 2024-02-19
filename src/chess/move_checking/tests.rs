#[cfg(test)]
mod tests {
    use crate::chess::move_checking::is_move_legal;
    use crate::chess::Color::*;
    use crate::chess::File::*;
    use crate::chess::PieceType::*;
    use crate::chess::Rank::*;
    use crate::chess::{Board, Color, File, Move, Piece, PieceType, Position, Rank};

    // Pawn Moves

    #[test]
    fn test_pawn_double_move_white() {
        let board = Board::default();
        let m = Move::Normal {
            from: Position(A, _2),
            to: Position(A, _2),
        };

        assert!(is_move_legal(&board, m));
    }

    #[test]
    fn test_pawn_double_move_black() {
        let mut board = Board::default();
        board.active_player = Black;
        let m = Move::Normal {
            from: Position(A, _7),
            to: Position(A, _5),
        };

        assert!(is_move_legal(&board, m));
    }

    #[test]
    fn test_cannot_move_other_player_pieces() {
        let board = Board::default();
        let m = Move::Normal {
            from: Position(A, _7),
            to: Position(A, _5),
        };

        assert!(!is_move_legal(&board, m));
    }

    //todo parameterize player, direction
    #[test]
    fn test_pawn_capture_right() {
        let mut board = Board::default();
        board.set_piece(A, _2, Piece(Pawn, White));
        board.set_piece(B, _3, Piece(Pawn, White));
        let m = Move::Normal {
            from: Position(A, _2),
            to: Position(B, _3),
        };

        assert!(is_move_legal(&board, m));
    }

    #[test]
    fn test_pawn_straight_move() {}

    #[test]
    fn test_pawn_straight_move_blocked() {}

    #[test]
    fn test_promotion() {}

    #[test]
    fn test_promotion_capture() {}

    #[test]
    fn test_en_passant() {}

    #[test]
    fn test_invalid_en_passant() {}

    // Bishop Moves
    #[test]
    fn test_bishop_legal_move() {}

    #[test]
    fn test_bishop_illegal_move() {}

    #[test]
    fn test_bishop_capture() {}

    // Knight Moves
    // Rook Moves
    // Queen Moves
    // King Moves
}
