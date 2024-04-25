

use crate::board::{
    model_utils::ColorProps,
    models::{Color, File, LegalMove, Piece, PieceType, Square},
    move_checking::square_utils::SquareIter,
    Board,
};

#[cfg(test)]
mod tests;
mod transposition_table;
mod zobrist_keys;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TranspEntry {
    pub depth: u8,
    pub value: f32,
}

pub struct TranspTable {
    // TODO add eviction policy
    table: Vec<Option<(u64, TranspEntry)>>,
    size: usize,
    occupancy: usize,
}

fn get_piece_square_key(piece: Piece, square: Square) -> u64 {
    zobrist_keys::PIECE_SQUARE_KEYS[piece.1 as usize][piece.0 as usize][square.0 as usize]
        [square.1 as usize]
}

pub fn get_zobrist_hash(board: &Board) -> u64 {
    let mut hash = 0;
    for square in SquareIter::new() {
        if let Some(piece) = board.get_piece_at(square) {
            hash ^= get_piece_square_key(piece, square);
        }
    }
    hash ^= zobrist_keys::CASTLING_KEYS[board.castling_rights as usize];
    if board.active_player == Color::Black {
        hash ^= zobrist_keys::BLACK_TO_MOVE_KEY;
    }
    if let Some(en_passant_square) = board.en_passant_target {
        hash ^= zobrist_keys::EN_PASSANT_KEYS[en_passant_square.0 as usize];
    }
    hash
}

// Precondition: move has not yet been applied to board
pub fn update_zobrist_hash(board: &Board, mut board_hash: u64, move_: &LegalMove) -> u64 {
    if let Some(Square(target_file, _)) = board.en_passant_target {
        board_hash ^= zobrist_keys::EN_PASSANT_KEYS[target_file as usize];
    }
    match move_ {
        LegalMove::Normal {
            src,
            dest,
            castle_mask,
            captured_piece,
        } => {
            let src_piece = board.get_piece_at(*src).unwrap();
            board_hash ^= get_piece_square_key(src_piece, *dest);
            board_hash ^= get_piece_square_key(src_piece, *src);
            if let Some(captured_piece) = captured_piece {
                board_hash ^= get_piece_square_key(
                    Piece(*captured_piece, board.active_player.opponent()),
                    *dest,
                );
            }
            board_hash ^= zobrist_keys::CASTLING_KEYS[board.castling_rights as usize];
            board_hash ^=
                zobrist_keys::CASTLING_KEYS[(board.castling_rights ^ castle_mask) as usize];
        }
        LegalMove::DoublePawnPush { file } => {
            board_hash ^= get_piece_square_key(
                Piece(PieceType::Pawn, board.active_player),
                Square(*file, board.active_player.pawn_start_rank()),
            );
            board_hash ^= get_piece_square_key(
                Piece(PieceType::Pawn, board.active_player),
                Square(*file, board.active_player.double_push_rank()),
            );
            board_hash ^= zobrist_keys::EN_PASSANT_KEYS[*file as usize];
        }
        LegalMove::CastleKingside { castle_mask } => {
            board_hash ^= get_piece_square_key(
                Piece(PieceType::King, board.active_player),
                Square(File::E, board.active_player.home_rank()),
            );
            board_hash ^= get_piece_square_key(
                Piece(PieceType::King, board.active_player),
                Square(File::G, board.active_player.home_rank()),
            );
            board_hash ^= get_piece_square_key(
                Piece(PieceType::Rook, board.active_player),
                Square(File::H, board.active_player.home_rank()),
            );
            board_hash ^= get_piece_square_key(
                Piece(PieceType::Rook, board.active_player),
                Square(File::F, board.active_player.home_rank()),
            );
            board_hash ^= zobrist_keys::CASTLING_KEYS[board.castling_rights as usize];
            board_hash ^=
                zobrist_keys::CASTLING_KEYS[(board.castling_rights ^ castle_mask) as usize];
        }
        LegalMove::CastleQueenside { castle_mask } => {
            board_hash ^= get_piece_square_key(
                Piece(PieceType::King, board.active_player),
                Square(File::E, board.active_player.home_rank()),
            );
            board_hash ^= get_piece_square_key(
                Piece(PieceType::King, board.active_player),
                Square(File::C, board.active_player.home_rank()),
            );
            board_hash ^= get_piece_square_key(
                Piece(PieceType::Rook, board.active_player),
                Square(File::A, board.active_player.home_rank()),
            );
            board_hash ^= get_piece_square_key(
                Piece(PieceType::Rook, board.active_player),
                Square(File::D, board.active_player.home_rank()),
            );
            board_hash ^= zobrist_keys::CASTLING_KEYS[board.castling_rights as usize];
            board_hash ^=
                zobrist_keys::CASTLING_KEYS[(board.castling_rights ^ castle_mask) as usize];
        }
        LegalMove::Promotion {
            src,
            dest,
            castle_mask,
            promotion,
            captured_piece,
        } => {
            board_hash ^= get_piece_square_key(Piece(PieceType::Pawn, board.active_player), *src);
            board_hash ^= get_piece_square_key(Piece(*promotion, board.active_player), *dest);
            if let Some(captured_piece) = captured_piece {
                board_hash ^= get_piece_square_key(
                    Piece(*captured_piece, board.active_player.opponent()),
                    *dest,
                );
            }
            board_hash ^= zobrist_keys::CASTLING_KEYS[board.castling_rights as usize];
            board_hash ^=
                zobrist_keys::CASTLING_KEYS[(board.castling_rights ^ castle_mask) as usize];
        }
        LegalMove::EnPassantCapture { src, dest } => {
            board_hash ^= get_piece_square_key(Piece(PieceType::Pawn, board.active_player), *src);
            board_hash ^= get_piece_square_key(Piece(PieceType::Pawn, board.active_player), *dest);
            board_hash ^= get_piece_square_key(
                Piece(PieceType::Pawn, board.active_player.opponent()),
                Square(dest.0, board.active_player.double_push_rank()),
            );
        }
    }
    board_hash ^= zobrist_keys::BLACK_TO_MOVE_KEY;
    board_hash
}
