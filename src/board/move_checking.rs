pub mod square_utils;
#[cfg(test)]
mod tests;

use crate::board::{Board, Move};

use self::square_utils::{
    is_move_pseudo_legal, pos_plus, DirIter, KnightHopIter, RayIter, SlideIter,
};

use super::{
    board_utils::is_square_attacked, model_utils::{ColorProps, PromotionToPiece}, models::LegalMove, Color, File, Piece, PieceType, PromotionPieceType, Rank, Square
};

fn check_move_blocked(
    piece: PieceType,
    src: Square,
    dest: Square,
    board: &Board,
) -> Result<(), String> {
    // All pieces: cannot move to a square occupied by a piece of the same color
    // this also filters null moves (src == dest)
    board.get_piece_at(dest).map_or(Ok(()), |dest_piece| {
        if dest_piece.1 == board.active_player {
            Err("Tried to move to a square occupied by a piece of the same color".to_string())
        } else {
            Ok(())
        }
    })?;

    if piece == PieceType::Knight || piece == PieceType::King {
        return Ok(());
    }

    if piece == PieceType::Pawn {
        // If moving sideways, must capture a piece (special case: en passant)
        if src.0 != dest.0
            && board.get_piece_at(dest).is_none()
            && board.en_passant_target != Some(dest)
        {
            return Err("Pawn cannot move sideways without capturing".to_string());
        }
        // If moving forward, must not be blocked
        if src.0 == dest.0 && board.get_piece_at(dest).is_some() {
            return Err("Pawn cannot move forward through occupied square".to_string());
        }
    }

    // Rook, Bishop, Queen: cannot move through occupied squares
    // Also checks long pawn move
    let slide_iter = SlideIter::new(src, dest);
    for pos in slide_iter {
        if board.get_piece_at(pos).is_some() {
            return Err("Sliding move is blocked".to_string());
        }
    }
    Ok(())
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

// Returns Some if move is an en passant capture
fn try_get_en_passant_capture(new_board: &mut Board, src: Square) -> Option<LegalMove> {
    // if move wasn't en passant capture, return
    let en_passant_target = match new_board.en_passant_target {
        Some(pos) => pos,
        None => return None, // no en passant target, thus no en passant capture
    };

    if !new_board
        .get_piece_at(en_passant_target)
        .map(|p| p.0 == PieceType::Pawn)
        .unwrap_or_default()
    {
        return None; // not a pawn move
    }

    // en passant capture, remove captured pawn
    let captured_pawn_pos = match new_board.active_player {
        Color::White => Square(en_passant_target.0, Rank::_5),
        Color::Black => Square(en_passant_target.0, Rank::_4),
    };
    new_board.clear_square(captured_pawn_pos); // remove pawn for subsequent king check in outer function
                                               //TODO fix this, ugly af, function responsibilities are unclear

    Some(LegalMove::EnPassantCapture {
        src,
        dest: en_passant_target,
    })
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

fn is_double_pawn_push(board: &Board, src: Square, dest: Square) -> bool {
    if board.get_piece_at(src).unwrap().0 != PieceType::Pawn {
        return false;
    }
    let (home_rank, hop_rank) = match board.active_player {
        Color::White => (Rank::_2, Rank::_4),
        Color::Black => (Rank::_7, Rank::_5),
    };
    src.1 == home_rank && dest.1 == hop_rank
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

    if check_move_blocked(src_piece.0, src, dest, board).is_err() {
        return None;
    }

    // carry out move
    // TODO: possible perf optimization - make board mutable and undo to avoid cloning
    let mut new_board = *board;
    new_board.set_piece_at(dest, src_piece);
    new_board.clear_square(src);

    let en_passant = try_get_en_passant_capture(&mut new_board, src);

    // check if king in check
    if is_king_in_check(&new_board) {
        return None;
    }

    if en_passant.is_some() {
        return en_passant;
    }

    // double pawn push

    if is_double_pawn_push(board, src, dest) {
        return Some(LegalMove::DoublePawnPush { file: src.0 });
    }

    let castle_mask = get_castling_mask(board, src, dest);
    let captured_piece = board.get_piece_at(dest).map(|p| p.0);

    Some(LegalMove::Normal {
        src,
        dest,
        castle_mask,
        captured_piece,
    })
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
    if is_king_in_check(board) {
        return false;
    }
    let home_rank = match board.active_player {
        Color::White => Rank::_1,
        Color::Black => Rank::_8,
    };
    let (king_pos, f_square, g_square, _rook_pos) = (
        Square(File::E, home_rank),
        Square(File::F, home_rank),
        Square(File::G, home_rank),
        Square(File::H, home_rank),
    );
    let mut new_board = *board;
    new_board.clear_square(king_pos);
    // F square must be empty and not under attack
    if board.get_piece_at(f_square).is_some() {
        return false;
    }
    new_board.set_piece_at(f_square, Piece(PieceType::King, board.active_player));
    if is_king_in_check(&new_board) {
        return false;
    }
    new_board.clear_square(f_square);
    // G square must be empty and not under attack
    if board.get_piece_at(g_square).is_some() {
        return false; // TODO avoid cloning board, use square attack helper function instead
    }
    new_board.set_piece_at(g_square, Piece(PieceType::King, board.active_player));
    if is_king_in_check(&new_board) {
        return false;
    }
    true
}

pub fn can_castle_queenside(board: &Board) -> bool {
    // must have castling rights
    if !board.has_queenside_castling_rights(board.active_player) {
        return false;
    }
    // must not be in check
    if is_king_in_check(board) {
        return false;
    }
    let home_rank = match board.active_player {
        Color::White => Rank::_1,
        Color::Black => Rank::_8,
    };
    let (king_pos, d_square, c_square, b_square) = (
        Square(File::E, home_rank),
        Square(File::D, home_rank),
        Square(File::C, home_rank),
        Square(File::B, home_rank),
    );
    let mut new_board = *board;
    new_board.clear_square(king_pos);
    // D square must be empty and not under attack
    if board.get_piece_at(d_square).is_some() {
        return false;
    }
    new_board.set_piece_at(d_square, Piece(PieceType::King, board.active_player));
    if is_king_in_check(&new_board) {
        return false;
    }
    new_board.clear_square(d_square);
    // C square must be empty and not under attack
    if board.get_piece_at(c_square).is_some() {
        return false;
    }
    new_board.set_piece_at(c_square, Piece(PieceType::King, board.active_player));
    if is_king_in_check(&new_board) {
        return false;
    }
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

// make move, mutates board
pub fn make_move(board: &mut Board, move_: &Move) -> Result<Board, String> {
    match get_legal_move_from_move(board, move_) {
        Some(legal_move) => Ok(apply_legal_move(board, &legal_move)),
        None => Err("Move is not legal".to_string()),
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
