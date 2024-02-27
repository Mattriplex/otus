pub mod square_utils;
#[cfg(test)]
mod tests;

use crate::board::{Board, Move};

use self::square_utils::{is_move_pseudo_legal, SlideIter};

use super::{
    board_utils::is_square_attacked,
    model_utils::{ColorProps, PromotionToPiece},
    models::LegalMove,
    Color, File, Piece, PieceType, PromotionPieceType, Rank, Square,
};

// To be used with bishop, rook, queen
// returns true if squares between src and dest are free of pieces (exclusive)
fn is_sliding_path_free(board: &Board, src: Square, dest: Square) -> bool {
    let slide_iter = SlideIter::new(src, dest);
    for pos in slide_iter {
        if board.get_piece_at(pos).is_some() {
            return false;
        }
    }
    true
}

fn seek_king(board: &Board, color: Color) -> Square {
    // search home row first, king is most likely there
    for rank in [
        color.home_rank(),
        Rank::_2,
        Rank::_7,
        Rank::_3,
        Rank::_4,
        Rank::_5,
        Rank::_6,
        color.opp_home_rank(),
    ] {
        for file in 0..8 {
            let pos = Square(File::from_i8(file).unwrap(), rank);
            if let Some(Piece(PieceType::King, c)) = board.get_piece_at(pos) {
                if c == color {
                    return pos;
                }
            }
        }
    }
    unreachable!("No king on the board");
}

// new_board: Move is already carried out, but active player is not switched
pub fn is_king_in_check(new_board: &Board) -> bool {
    let king_pos = seek_king(new_board, new_board.active_player);
    is_square_attacked(new_board, king_pos)
}

// this function does not check if the pawn belongs to the active player, handle_normal_move does that
pub fn is_promotion_move(board: &Board, src: Square, dest: Square) -> bool {
    match board.get_piece_at(src) {
        Some(Piece(PieceType::Pawn, Color::White)) => dest.1 == Rank::_8,
        Some(Piece(PieceType::Pawn, Color::Black)) => dest.1 == Rank::_1,
        _ => false,
    }
}

pub fn apply_move(board: &Board, move_: &Move) -> Result<Board, String> {
    match get_legal_move_from_move(board, move_) {
        Some(legal_move) => Ok(apply_legal_move(board, &legal_move)),
        None => Err("Move is not legal".to_string()),
    }
}

fn get_castling_mask(old_board: &Board, src: Square, dest: Square) -> u8 {
    // short-circuit if no castling rights to update
    if old_board.castling_rights == 0b0000 {
        return 0b0000;
    }
    let mut mask: u8 = 0b0000;
    // does the move capture the opponent's rook?
    let (home_rank, opp_home_rank) = match old_board.active_player {
        Color::White => (Rank::_1, Rank::_8),
        Color::Black => (Rank::_8, Rank::_1),
    };
    if dest == Square(File::A, opp_home_rank)
        && old_board.has_queenside_castling_rights(old_board.active_player.opponent())
    {
        mask |= match old_board.active_player {
            Color::White => 0b0001,
            Color::Black => 0b0100,
        };
    } else if dest == Square(File::H, opp_home_rank)
        && old_board.has_kingside_castling_rights(old_board.active_player.opponent())
    {
        mask |= match old_board.active_player {
            Color::White => 0b0010,
            Color::Black => 0b1000,
        };
    }
    if old_board.has_kingside_castling_rights(old_board.active_player)
        || old_board.has_queenside_castling_rights(old_board.active_player)
    {
        // does the move move the king?
        if src == Square(File::E, home_rank) {
            mask |= match old_board.active_player {
                Color::White => 0b1100,
                Color::Black => 0b0011,
            };
        } else if src == Square(File::H, home_rank) {
            // does the move move a rook?

            mask |= match old_board.active_player {
                Color::White => 0b1000,
                Color::Black => 0b0010,
            };
        } else if src == Square(File::A, home_rank) {
            mask |= match old_board.active_player {
                Color::White => 0b0100,
                Color::Black => 0b0001,
            };
        }
    }
    mask
}

fn get_normal_legal_move_from_pseudolegal(
    board: &Board,
    src: Square,
    dest: Square,
) -> Option<LegalMove> {
    let src_piece = match board.get_piece_at(src) {
        Some(piece) => piece,
        None => return None, // No piece at source
    };

    if src_piece.1 != board.active_player {
        return None; // tried to move opponent's piece
    }

    // cannot move to square occupied by my own piece
    let dst_piece = board.get_piece_at(dest);
    if dst_piece.map_or(false, |p| p.1 == board.active_player) {
        return None;
    }
    let normal_move = || LegalMove::Normal {
        src,
        dest,
        castle_mask: get_castling_mask(board, src, dest),
        captured_piece: dst_piece.map(|p| p.0),
    };
    let legal_move = match src_piece.0 {
        PieceType::Queen | PieceType::Rook | PieceType::Bishop => {
            if !is_sliding_path_free(board, src, dest) {
                return None;
            }
            normal_move()
        }
        PieceType::Knight | PieceType::King => normal_move(),
        PieceType::Pawn => {
            if src.0 != dest.0 {
                // Diagonal move, dest must contain a piece or be en passant target
                if board.get_piece_at(dest).is_none() {
                    // dest is empty, check for en passant
                    if board.en_passant_target != Some(dest) {
                        return None; // Invalid move, neither en passant or capture
                    }
                    LegalMove::EnPassantCapture { src, dest }
                } else {
                    // normal capture
                    normal_move()
                }
            } else {
                // Forward move, dest must not contain a piece
                if board.get_piece_at(dest).is_some() {
                    return None;
                }
                if src.1 == board.active_player.pawn_start_rank()
                    && dest.1 == board.active_player.double_push_rank()
                {
                    // double pawn push, check if square in front is occupied
                    if board
                        .get_piece_at(Square(src.0, board.active_player.hop_rank()))
                        .is_some()
                    {
                        return None;
                    }
                    LegalMove::DoublePawnPush { file: src.0 }
                } else {
                    LegalMove::Normal {
                        src,
                        dest,
                        castle_mask: 0,
                        captured_piece: None,
                    }
                }
            }
        }
    };

    // carry out move
    // TODO: possible perf optimization - make board mutable and undo to avoid cloning
    let mut new_board = *board;
    new_board.make_move(&legal_move);
    new_board.active_player = new_board.active_player.opponent(); // undo player switching done by make_move

    // check if king in check
    if is_king_in_check(&new_board) {
        return None;
    }

    Some(legal_move)
}

fn get_promotion_legal_move_from_pseudolegal(
    board: &Board,
    src: Square,
    dest: Square,
    promotion: PromotionPieceType,
) -> Option<LegalMove> {
    let (castle_mask, captured_piece) =
        match get_normal_legal_move_from_pseudolegal(board, src, dest) {
            Some(LegalMove::Normal {
                castle_mask,
                captured_piece,
                ..
            }) => (castle_mask, captured_piece),
            _ => return None, // move is not legal
        };
    Some(LegalMove::Promotion {
        src,
        dest,
        castle_mask,
        promotion: promotion.to_piece(),
        captured_piece,
    })
}

// TODO clean up
pub fn can_castle_kingside(board: &Board) -> bool {
    // must have castling rights
    if !board.has_kingside_castling_rights(board.active_player) {
        return false;
    }
    // must not be in check
    // we know king is on home square because castling rights are intact
    // using is_square attacked instead of is_king_in_check to save a call to seek_king
    if is_square_attacked(board, board.active_player.king_home_square()) {
        return false;
    }
    let home_rank = board.active_player.home_rank();
    let (f_square, g_square) = (Square(File::F, home_rank), Square(File::G, home_rank));
    // F and G square must be empty and not under attack
    if board.get_piece_at(f_square).is_some() || is_square_attacked(board, f_square) {
        return false;
    }
    if board.get_piece_at(g_square).is_some() || is_square_attacked(board, g_square) {
        return false; // TODO avoid cloning board, use square attack helper function instead
    }
    true
}

pub fn can_castle_queenside(board: &Board) -> bool {
    // must have castling rights
    if !board.has_queenside_castling_rights(board.active_player) {
        return false;
    }
    // must not be in check
    // we know king is on home square because castling rights are intact
    // using is_square attacked instead of is_king_in_check to save a call to seek_king    if is_king_in_check(board) {
    if is_square_attacked(board, board.active_player.king_home_square()) {
        return false;
    }
    let home_rank = board.active_player.home_rank();
    let (d_square, c_square, b_square) = (
        Square(File::D, home_rank),
        Square(File::C, home_rank),
        Square(File::B, home_rank),
    );

    // D square must be empty and not under attack
    if board.get_piece_at(d_square).is_some() || is_square_attacked(board, d_square) {
        return false;
    }
    // C square must be empty and not under attack
    if board.get_piece_at(c_square).is_some() || is_square_attacked(board, c_square) {
        return false;
    }
    // B sqare must be empty
    if board.get_piece_at(b_square).is_some() {
        return false;
    }
    true
}

pub fn get_legal_move_from_move(board: &Board, move_: &Move) -> Option<LegalMove> {
    match move_ {
        Move::Normal { src, dest } => {
            if is_promotion_move(board, *src, *dest) {
                None // missing promotion piece
            } else if let Some(Piece(src_piece, owner)) = board.get_piece_at(*src) {
                if is_move_pseudo_legal(*src, *dest, src_piece, owner) {
                    get_normal_legal_move_from_pseudolegal(board, *src, *dest)
                } else {
                    None
                }
            } else {
                None
            }
        }
        Move::CastleKingside { .. } => {
            if can_castle_kingside(board) {
                Some(LegalMove::CastleKingside {
                    castle_mask: board.castling_rights & board.active_player.castle_bit_mask(),
                })
            } else {
                None
            }
        }
        Move::CastleQueenside { .. } => {
            if can_castle_queenside(board) {
                Some(LegalMove::CastleQueenside {
                    castle_mask: board.castling_rights & board.active_player.castle_bit_mask(),
                })
            } else {
                None
            }
        }
        Move::Promotion {
            src,
            dest,
            promotion,
        } => get_promotion_legal_move_from_pseudolegal(board, *src, *dest, *promotion),
    }
}

/*
Assumes move is pseudo legal, i.e.
1. a piece of the active player stands on src
2. the movement pattern agrees with that piece. Double pawn pushes are only possible from active player's home rank
*/
pub fn get_legal_move_from_pseudolegal_move(board: &Board, move_: &Move) -> Option<LegalMove> {
    match move_ {
        Move::Normal { src, dest } => {
            // TODO remove this check, should never happen
            if is_promotion_move(board, *src, *dest) {
                None // missing promotion piece
            } else {
                get_normal_legal_move_from_pseudolegal(board, *src, *dest)
            }
        }
        Move::CastleKingside { .. } => {
            if can_castle_kingside(board) {
                Some(LegalMove::CastleKingside {
                    castle_mask: board.castling_rights & board.active_player.castle_bit_mask(),
                })
            } else {
                None
            }
        }
        Move::CastleQueenside { .. } => {
            if can_castle_queenside(board) {
                Some(LegalMove::CastleQueenside {
                    castle_mask: board.castling_rights & board.active_player.castle_bit_mask(),
                })
            } else {
                None
            }
        }
        Move::Promotion {
            src,
            dest,
            promotion,
        } => {
            if is_promotion_move(board, *src, *dest) {
                get_promotion_legal_move_from_pseudolegal(board, *src, *dest, *promotion)
            } else {
                None
            }
        }
    }
}

pub fn apply_legal_move(board: &Board, move_: &LegalMove) -> Board {
    let mut new_board = *board;
    new_board.make_move(move_);
    new_board
}

pub fn is_move_legal(board: &Board, move_: &Move) -> bool {
    apply_move(board, move_).is_ok()
}
