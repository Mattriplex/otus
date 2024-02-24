pub mod square_utils;
#[cfg(test)]
mod tests;

use crate::board::{Board, Move};

use self::square_utils::{
    is_move_pseudo_legal, pos_plus, DirIter, KnightHopIter, RayIter, SlideIter,
};

use super::{
    model_utils::Opponent, models::LegalMove, Color, File, Piece, PieceType, PromotionPieceType,
    Rank, Square,
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
        if src.0 != dest.0 {
            if board.get_piece_at(dest).is_none() && board.en_passant_target.is_none() {
                return Err("Pawn cannot move sideways without capturing".to_string());
            }
        }
        // If moving forward, must not be blocked
        if src.0 == dest.0 {
            if board.get_piece_at(dest).is_some() {
                return Err("Pawn cannot move forward through occupied square".to_string());
            }
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

// Precondition: pawn move already carried out
fn handle_en_passant_move(new_board: &mut Board) {
    // if move wasn't en passant capture, return
    let en_passant_target = match new_board.en_passant_target {
        Some(pos) => pos,
        None => return, // no en passant target, thus no en passant capture
    };

    if let Some(Piece(PieceType::Pawn, color)) = new_board.get_piece_at(en_passant_target) {
        if color != new_board.active_player {
            unreachable!("En passant target is not a pawn of the active player")
        }
    } else {
        return; // not a pawn move
    }

    // en passant capture, remove captured pawn
    let captured_pawn_pos = match new_board.active_player {
        Color::White => pos_plus(en_passant_target, (0, -1)),
        Color::Black => pos_plus(en_passant_target, (0, 1)),
    }
    .expect("En passant target neighbour out of bounds");
    new_board.clear_square(captured_pawn_pos);
    new_board.en_passant_target = None;
}

fn seek_king(board: &Board, color: Color) -> Option<Square> {
    for file in 0..8 {
        for rank in 0..8 {
            let pos = Square(File::from_i8(file).unwrap(), Rank::from_i8(rank).unwrap());
            if let Some(Piece(PieceType::King, c)) = board.get_piece_at(pos) {
                if c == color {
                    return Some(pos);
                }
            }
        }
    }
    return None;
}

// new_board: Move is already carried out, but active player is not switched
pub fn is_king_in_check(new_board: &Board) -> bool {
    let king_pos = match seek_king(new_board, new_board.active_player) {
        Some(pos) => pos,
        None => return false, // no king on the board
    };

    // check for pawn checks
    let pawn_dir = match new_board.active_player {
        Color::White => 1,
        Color::Black => -1,
    };
    for pos in [(1, pawn_dir), (-1, pawn_dir)]
        .iter()
        .filter_map(|step| pos_plus(king_pos, *step))
    {
        if let Some(Piece(PieceType::Pawn, color)) = new_board.get_piece_at(pos) {
            if color != new_board.active_player {
                return true;
            }
        }
    }

    // cast rays from king to check for threats
    for dir in DirIter::rook() {
        for pos in RayIter::new(king_pos, dir) {
            if let Some(Piece(piece, color)) = new_board.get_piece_at(pos) {
                let is_rook_or_queen = piece == PieceType::Rook || piece == PieceType::Queen;
                if color == new_board.active_player || !is_rook_or_queen {
                    break;
                }
                if is_rook_or_queen {
                    return true;
                }
            }
        }
    }
    for dir in DirIter::bishop() {
        for pos in RayIter::new(king_pos, dir) {
            if let Some(Piece(piece, color)) = new_board.get_piece_at(pos) {
                let is_bishop_or_queen = piece == PieceType::Bishop || piece == PieceType::Queen;
                if color == new_board.active_player || !is_bishop_or_queen {
                    break;
                }
                if is_bishop_or_queen {
                    return true;
                }
            }
        }
    }

    // check king moves
    for pos in DirIter::all().filter_map(|dir| pos_plus(king_pos, dir)) {
        if let Some(Piece(PieceType::King, color)) = new_board.get_piece_at(pos) {
            if color != new_board.active_player {
                return true;
            }
        }
    }

    // check knight moves
    for pos in KnightHopIter::new(king_pos) {
        if let Some(Piece(PieceType::Knight, color)) = new_board.get_piece_at(pos) {
            if color != new_board.active_player {
                return true;
            }
        }
    }

    return false;
}

fn update_castling_rights(new_board: &mut Board, src: Square, dest: Square) {
    // short-circuit if no castling rights to update
    if new_board.castling_rights == 0b0000 {
        return;
    }
    let active_player = new_board.active_player;
    // capturing the opponent's rook removes their castling rights
    let (home_rank, opp_home_rank) = match active_player {
        Color::White => (Rank::_1, Rank::_8),
        Color::Black => (Rank::_8, Rank::_1),
    };
    if dest.clone() == Square(File::A, opp_home_rank) {
        new_board.revoke_queenside_castling(active_player.opponent())
    } else if dest.clone() == Square(File::H, opp_home_rank) {
        new_board.revoke_kingside_castling(active_player.opponent())
    }
    // moving the king removes castling rights
    let (can_kingside_castle, can_queenside_castle) = (
        new_board.has_kingside_castling_rights(active_player),
        new_board.has_queenside_castling_rights(active_player),
    );
    if (can_kingside_castle || can_queenside_castle) && src.clone() == Square(File::E, home_rank) {
        new_board.revoke_kingside_castling(active_player);
        new_board.revoke_queenside_castling(active_player);
    }
    // moving a rook removes castling rights
    if can_kingside_castle && src.clone() == Square(File::H, home_rank) {
        new_board.revoke_kingside_castling(active_player);
    }
    if can_queenside_castle && src.clone() == Square(File::A, home_rank) {
        new_board.revoke_queenside_castling(active_player);
    }
}

fn update_en_passant_square(new_board: &mut Board, src: Square, dest: Square) {
    let moved_piece = match new_board.get_piece_at(dest) {
        Some(piece) => piece.0,
        None => unreachable!(),
    };
    let (home_rank, target_rank, hop_rank) = match new_board.active_player {
        Color::White => (Rank::_2, Rank::_3, Rank::_4),
        Color::Black => (Rank::_7, Rank::_6, Rank::_5),
    };
    if moved_piece == PieceType::Pawn && src.1 == home_rank && dest.1 == hop_rank {
        new_board.en_passant_target = Some(Square(dest.0, target_rank));
    } else {
        new_board.en_passant_target = None;
    }
}

fn handle_normal_move(board: &Board, src: Square, dest: Square) -> Result<Board, String> {
    let src_piece = match board.get_piece_at(src) {
        Some(piece) => piece,
        None => return Err("No piece at move origin".to_string()),
    };

    if src_piece.1 != board.active_player {
        return Err("Tried to move opponent's piece".to_string());
    }

    if !is_move_pseudo_legal(src, dest, src_piece.0, board.active_player) {
        return Err(format!("Piece {} cannot move this way", src_piece));
    }
    check_move_blocked(src_piece.0, src, dest, board)?;

    // carry out move
    let mut new_board = board.clone();
    new_board.set_piece_at(dest, src_piece);
    new_board.clear_square(src);

    handle_en_passant_move(&mut new_board);
    // check if king in check
    if is_king_in_check(&new_board) {
        return Err("Move would leave king in check".to_string());
    }

    update_castling_rights(&mut new_board, src, dest);
    update_en_passant_square(&mut new_board, src, dest);
    new_board.active_player = board.active_player.opponent();

    Ok(new_board)
}

// this function does not check if the pawn belongs to the active player, handle_normal_move does that
pub fn is_promotion_move(board: &Board, src: Square, dest: Square) -> bool {
    match board.get_piece_at(src) {
        Some(Piece(PieceType::Pawn, Color::White)) => dest.1 == Rank::_8,
        Some(Piece(PieceType::Pawn, Color::Black)) => dest.1 == Rank::_1,
        _ => false,
    }
}

fn handle_promotion_move(board: &Board, move_: &Move) -> Result<Board, String> {
    let (src, dest, promotion) = match move_ {
        Move::Promotion {
            src: from,
            dest: to,
            promotion,
        } => (from, to, promotion),
        _ => unreachable!(),
    };

    if !is_promotion_move(board, *src, *dest) {
        return Err("Not a promotion move".to_string());
    }

    let mut new_board = handle_normal_move(board, *src, *dest)?;

    // replace pawn with promoted piece
    let promotion_type = match promotion {
        PromotionPieceType::Queen => PieceType::Queen,
        PromotionPieceType::Rook => PieceType::Rook,
        PromotionPieceType::Bishop => PieceType::Bishop,
        PromotionPieceType::Knight => PieceType::Knight,
    };
    new_board.set_piece_at(*dest, Piece(promotion_type, board.active_player));

    Ok(new_board)
}

fn check_and_handle_normal_move(board: &Board, src: Square, dest: Square) -> Result<Board, String> {
    if is_promotion_move(board, src, dest) {
        Err("Move is a promotion but no piece was specified".to_string())
    } else {
        handle_normal_move(board, src, dest)
    }
}

fn handle_kingside_castle(board: &Board) -> Result<Board, String> {
    // must have castling rights
    if !board.has_kingside_castling_rights(board.active_player) {
        return Err("No castling rights".to_string());
    }
    // must not be in check
    if is_king_in_check(board) {
        return Err("Cannot castle while in check".to_string());
    }
    let home_rank = match board.active_player {
        Color::White => Rank::_1,
        Color::Black => Rank::_8,
    };
    let (king_pos, f_square, g_square, rook_pos) = (
        Square(File::E, home_rank),
        Square(File::F, home_rank),
        Square(File::G, home_rank),
        Square(File::H, home_rank),
    );
    let mut new_board = board.clone();
    new_board.clear_square(king_pos);
    // F square must be empty and not under attack
    if let Some(_) = board.get_piece_at(f_square) {
        return Err("Cannot castle through occupied square".to_string());
    }
    new_board.set_piece_at(f_square, Piece(PieceType::King, board.active_player));
    if is_king_in_check(&new_board) {
        return Err("Cannot castle through check".to_string());
    }
    new_board.clear_square(f_square);
    // G square must be empty and not under attack
    if let Some(_) = board.get_piece_at(g_square) {
        return Err("Cannot castle through occupied square".to_string());
    }
    new_board.set_piece_at(g_square, Piece(PieceType::King, board.active_player));
    if is_king_in_check(&new_board) {
        return Err("Cannot castle into check".to_string());
    }
    new_board.clear_square(rook_pos);
    new_board.set_piece_at(f_square, Piece(PieceType::Rook, board.active_player));
    new_board.revoke_kingside_castling(board.active_player);
    new_board.revoke_queenside_castling(board.active_player);
    new_board.active_player = board.active_player.opponent();
    new_board.en_passant_target = None;
    Ok(new_board)
}

fn handle_queenside_castle(board: &Board) -> Result<Board, String> {
    // must have castling rights
    if !board.has_queenside_castling_rights(board.active_player) {
        return Err("No castling rights".to_string());
    }
    // must not be in check
    if is_king_in_check(board) {
        return Err("Cannot castle while in check".to_string());
    }
    let home_rank = match board.active_player {
        Color::White => Rank::_1,
        Color::Black => Rank::_8,
    };
    let (king_pos, d_square, c_square, b_square, rook_pos) = (
        Square(File::E, home_rank),
        Square(File::D, home_rank),
        Square(File::C, home_rank),
        Square(File::B, home_rank),
        Square(File::A, home_rank),
    );
    let mut new_board = board.clone();
    new_board.clear_square(king_pos);
    // D square must be empty and not under attack
    if let Some(_) = board.get_piece_at(d_square) {
        return Err("Cannot castle through occupied square".to_string());
    }
    new_board.set_piece_at(d_square, Piece(PieceType::King, board.active_player));
    if is_king_in_check(&new_board) {
        return Err("Cannot castle through check".to_string());
    }
    new_board.clear_square(d_square);
    // C square must be empty and not under attack
    if let Some(_) = board.get_piece_at(c_square) {
        return Err("Cannot castle through occupied square".to_string());
    }
    new_board.set_piece_at(c_square, Piece(PieceType::King, board.active_player));
    if is_king_in_check(&new_board) {
        return Err("Cannot castle into check".to_string());
    }
    if let Some(_) = board.get_piece_at(b_square) {
        return Err("Cannot castle through occupied square".to_string());
    }
    new_board.clear_square(rook_pos);
    new_board.set_piece_at(d_square, Piece(PieceType::Rook, board.active_player));
    new_board.revoke_kingside_castling(board.active_player);
    new_board.revoke_queenside_castling(board.active_player);
    new_board.active_player = board.active_player.opponent();
    new_board.en_passant_target = None;
    Ok(new_board)
}

// TODO: switch active player
pub fn apply_move(board: &Board, move_: &Move) -> Result<Board, String> {
    match move_ {
        Move::Normal {
            src: from,
            dest: to,
        } => check_and_handle_normal_move(board, *from, *to),
        Move::CastleKingside { .. } => handle_kingside_castle(board),
        Move::CastleQueenside { .. } => handle_queenside_castle(board),
        Move::Promotion { .. } => handle_promotion_move(board, move_),
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

    Some(LegalMove::EnPassantCapture { src })
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
    return src.1 == home_rank && dest.1 == hop_rank;
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
    let mut new_board = board.clone();
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

    if is_double_pawn_push(&board, src, dest) {
        return Some(LegalMove::DoublePawnPush { file: src.0 });
    }

    let castling_mask = get_castling_mask(board, src, dest);

    Some(LegalMove::Normal {
        src,
        dest,
        castle_mask: castling_mask,
    })
}

fn get_promotion_legal_move_from_pseudolegal(
    board: &Board,
    src: Square,
    dest: Square,
    promotion: PromotionPieceType,
) -> Option<LegalMove> {
    let castlemask = match get_normal_legal_move_from_pseudolegal(board, src, dest) {
        Some(LegalMove::Normal { castle_mask, .. }) => castle_mask,
        _ => return None, // move is not legal
    };
    Some(LegalMove::Promotion {
        src,
        dest,
        castle_mask: castlemask,
        promotion,
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
    let (king_pos, f_square, g_square, rook_pos) = (
        Square(File::E, home_rank),
        Square(File::F, home_rank),
        Square(File::G, home_rank),
        Square(File::H, home_rank),
    );
    let mut new_board = board.clone();
    new_board.clear_square(king_pos);
    // F square must be empty and not under attack
    if let Some(_) = board.get_piece_at(f_square) {
        return false;
    }
    new_board.set_piece_at(f_square, Piece(PieceType::King, board.active_player));
    if is_king_in_check(&new_board) {
        return false;
    }
    new_board.clear_square(f_square);
    // G square must be empty and not under attack
    if let Some(_) = board.get_piece_at(g_square) {
        return false; // TODO avoid cloning board, use square attack helper function instead
    }
    new_board.set_piece_at(g_square, Piece(PieceType::King, board.active_player));
    if is_king_in_check(&new_board) {
        return false;
    }
    return true;
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
    let mut new_board = board.clone();
    new_board.clear_square(king_pos);
    // D square must be empty and not under attack
    if let Some(_) = board.get_piece_at(d_square) {
        return false;
    }
    new_board.set_piece_at(d_square, Piece(PieceType::King, board.active_player));
    if is_king_in_check(&new_board) {
        return false;
    }
    new_board.clear_square(d_square);
    // C square must be empty and not under attack
    if let Some(_) = board.get_piece_at(c_square) {
        return false;
    }
    new_board.set_piece_at(c_square, Piece(PieceType::King, board.active_player));
    if is_king_in_check(&new_board) {
        return false;
    }
    if let Some(_) = board.get_piece_at(b_square) {
        return false;
    }
    true
}

pub fn get_legal_move_from_move(board: &Board, move_: &Move) -> Option<LegalMove> {
    // apply psuedolegal checks, then run function below
    unimplemented!()
}

pub fn get_legal_move_from_pseudolegal_move(board: &Board, move_: &Move) -> Option<LegalMove> {
    match move_ {
        Move::Normal { src, dest } => get_normal_legal_move_from_pseudolegal(board, *src, *dest),
        Move::CastleKingside { .. } => {
            if can_castle_kingside(board) {
                Some(LegalMove::CastleKingside)
            } else {
                None
            }
        }
        Move::CastleQueenside { .. } => {
            if can_castle_queenside(board) {
                Some(LegalMove::CastleQueenside)
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

pub fn apply_legal_move(board: &Board, move_: &LegalMove) -> Board {
    let mut new_board = board.clone();
    new_board.en_passant_target = None;
    match move_ {
        LegalMove::Normal {
            src,
            dest,
            castle_mask,
        } => {
            let piece = new_board.get_piece_at(*src).unwrap();
            new_board.set_piece_at(*dest, piece);
            new_board.clear_square(*src);
            new_board.castling_rights ^= castle_mask;
        }
        LegalMove::Promotion {
            src,
            dest,
            castle_mask,
            promotion,
        } => {
            let piece = match promotion {
                PromotionPieceType::Queen => PieceType::Queen,
                PromotionPieceType::Rook => PieceType::Rook,
                PromotionPieceType::Bishop => PieceType::Bishop,
                PromotionPieceType::Knight => PieceType::Knight,
            };
            new_board.set_piece_at(*dest, Piece(piece, new_board.active_player));
            new_board.clear_square(*src);
            new_board.castling_rights ^= castle_mask;
        }
        LegalMove::CastleKingside => {
            let home_rank = match new_board.active_player {
                Color::White => Rank::_1,
                Color::Black => Rank::_8,
            };
            let (king_pos, f_square, g_square, rook_pos) = (
                Square(File::E, home_rank),
                Square(File::F, home_rank),
                Square(File::G, home_rank),
                Square(File::H, home_rank),
            );
            new_board.clear_square(king_pos);
            new_board.set_piece_at(g_square, Piece(PieceType::King, new_board.active_player));
            new_board.clear_square(rook_pos);
            new_board.set_piece_at(f_square, Piece(PieceType::Rook, new_board.active_player));
            new_board.castling_rights &= match new_board.active_player {
                Color::White => 0b0011,
                Color::Black => 0b1100,
            };
        }
        LegalMove::CastleQueenside => {
            let home_rank = match new_board.active_player {
                Color::White => Rank::_1,
                Color::Black => Rank::_8,
            };
            let (king_pos, d_square, c_square, rook_pos) = (
                Square(File::E, home_rank),
                Square(File::D, home_rank),
                Square(File::C, home_rank),
                Square(File::A, home_rank),
            );
            new_board.clear_square(king_pos);
            new_board.set_piece_at(c_square, Piece(PieceType::King, new_board.active_player));
            new_board.clear_square(rook_pos);
            new_board.set_piece_at(d_square, Piece(PieceType::Rook, new_board.active_player));
            new_board.castling_rights &= match new_board.active_player {
                Color::White => 0b0011,
                Color::Black => 0b1100,
            };
        }
        LegalMove::DoublePawnPush { file: f } => {
            let (src_rank, target_rank, dst_rank) = match new_board.active_player {
                Color::White => (Rank::_2, Rank::_3, Rank::_4),
                Color::Black => (Rank::_7, Rank::_6, Rank::_5),
            };
            new_board.set_piece_at(
                Square(*f, dst_rank),
                Piece(PieceType::Pawn, new_board.active_player),
            );
            new_board.clear_square(Square(*f, src_rank));
            new_board.en_passant_target = Some(Square(*f, target_rank));
        }
        LegalMove::EnPassantCapture { src } => {
            let en_passant_target = match new_board.en_passant_target {
                Some(pos) => pos,
                None => unreachable!("No en passant target"),
            };
            new_board.set_piece_at(en_passant_target, new_board.get_piece_at(*src).unwrap());
            new_board.clear_square(*src);
            let captured_pawn_pos = match new_board.active_player {
                Color::White => Square(en_passant_target.0, Rank::_5),
                Color::Black => Square(en_passant_target.0, Rank::_4),
            };
            new_board.clear_square(captured_pawn_pos);
        }
    }
    new_board.active_player = board.active_player.opponent();
    new_board
}

pub fn is_move_legal(board: &Board, move_: &Move) -> bool {
    match apply_move(board, move_) {
        Ok(_) => true,
        Err(_) => false,
    }
}
