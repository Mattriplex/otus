mod square_utils;
#[cfg(test)]
mod tests;

use crate::chess::{Board, Move};

use self::square_utils::{
    is_move_pseudo_legal, pos_plus, DirIter, KnightHopIter, RayIter, SlideIter,
};

use super::{Color, File, GameState, Opponent, Piece, PieceType, Pos, PromotionPieceType, Rank};

fn check_move_blocked(piece: PieceType, src: Pos, dest: Pos, board: &Board) -> Result<(), String> {
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
            if board.get_piece_at(dest).is_none() && board.en_passant_target != Some(dest) {
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

fn seek_king(board: &Board, color: Color) -> Option<Pos> {
    for file in 0..8 {
        for rank in 0..8 {
            let pos = Pos(File::from_i8(file).unwrap(), Rank::from_i8(rank).unwrap());
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
fn is_king_in_check(new_board: &Board) -> bool {
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

fn update_castling_rights(new_board: &mut Board, src: Pos, dest: Pos) {
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
    if dest.clone() == Pos(File::A, opp_home_rank) {
        new_board.revoke_queenside_castling(active_player.opponent())
    } else if dest.clone() == Pos(File::H, opp_home_rank) {
        new_board.revoke_kingside_castling(active_player.opponent())
    }
    // moving the king removes castling rights
    let (can_kingside_castle, can_queenside_castle) = (
        new_board.can_castle_kingside(active_player),
        new_board.can_castle_queenside(active_player),
    );
    if (can_kingside_castle || can_queenside_castle) && src.clone() == Pos(File::E, home_rank) {
        new_board.revoke_kingside_castling(active_player);
        new_board.revoke_queenside_castling(active_player);
    }
    // moving a rook removes castling rights
    if can_kingside_castle && src.clone() == Pos(File::H, home_rank) {
        new_board.revoke_kingside_castling(active_player);
    }
    if can_queenside_castle && src.clone() == Pos(File::A, home_rank) {
        new_board.revoke_queenside_castling(active_player);
    }
}

fn update_en_passant_square(new_board: &mut Board, src: Pos, dest: Pos) {
    let moved_piece = match new_board.get_piece_at(dest) {
        Some(piece) => piece.0,
        None => unreachable!(),
    };
    let (home_rank, target_rank, hop_rank) = match new_board.active_player {
        Color::White => (Rank::_2, Rank::_3, Rank::_4),
        Color::Black => (Rank::_7, Rank::_6, Rank::_5),
    };
    if moved_piece == PieceType::Pawn && src.1 == home_rank && dest.1 == hop_rank {
        new_board.en_passant_target = Some(Pos(dest.0, target_rank));
    } else {
        new_board.en_passant_target = None;
    }
}

fn handle_normal_move(board: &Board, src: Pos, dest: Pos) -> Result<Board, String> {
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
pub fn is_promotion_move(board: &Board, src: Pos, dest: Pos) -> bool {
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

fn check_and_handle_normal_move(board: &Board, src: Pos, dest: Pos) -> Result<Board, String> {
    if is_promotion_move(board, src, dest) {
        Err("Move is a promotion but no piece was specified".to_string())
    } else {
        handle_normal_move(board, src, dest)
    }
}

fn handle_kingside_castle(board: &Board) -> Result<Board, String> {
    // must have castling rights
    if !board.can_castle_kingside(board.active_player) {
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
        Pos(File::E, home_rank),
        Pos(File::F, home_rank),
        Pos(File::G, home_rank),
        Pos(File::H, home_rank),
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
    if !board.can_castle_queenside(board.active_player) {
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
        Pos(File::E, home_rank),
        Pos(File::D, home_rank),
        Pos(File::C, home_rank),
        Pos(File::B, home_rank),
        Pos(File::A, home_rank),
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

pub fn is_move_legal(board: &Board, move_: &Move) -> bool {
    match apply_move(board, move_) {
        Ok(_) => true,
        Err(_) => false,
    }
}

// TODO clean up
// step 1: get list of possible moves (depends on piece)
// step 2: filter illegal moves
pub fn get_legal_moves(board: &Board) -> Vec<Move> {
    let mut legal_moves = Vec::new();
    let (opp_home_rank, pawn_dir) = match board.active_player {
        Color::White => (Rank::_8, 1),
        Color::Black => (Rank::_1, -1),
    };
    for file in 0..8 {
        for rank in 0..8 {
            let src = Pos(File::from_i8(file).unwrap(), Rank::from_i8(rank).unwrap());
            let piece = match board.get_piece_at(src) {
                Some(piece) => piece,
                None => continue,
            };
            if piece.1 != board.active_player {
                continue;
            }
            if piece.0 == PieceType::Pawn {
                for dest in [
                    (0, pawn_dir),
                    (0, 2 * pawn_dir),
                    (1, pawn_dir),
                    (-1, pawn_dir),
                ]
                .iter()
                .filter_map(|step| pos_plus(src, *step))
                {
                    let move_;
                    if dest.1 == opp_home_rank {
                        move_ = Move::Promotion {
                            src,
                            dest,
                            promotion: PromotionPieceType::Queen,
                        };
                        if is_move_legal(board, &move_) {
                            legal_moves.push(move_);
                            legal_moves.push(Move::Promotion {
                                src,
                                dest,
                                promotion: PromotionPieceType::Rook,
                            });
                            legal_moves.push(Move::Promotion {
                                src,
                                dest,
                                promotion: PromotionPieceType::Bishop,
                            });
                            legal_moves.push(Move::Promotion {
                                src,
                                dest,
                                promotion: PromotionPieceType::Knight,
                            });
                        }
                    } else {
                        move_ = Move::Normal { src, dest };
                        if is_move_legal(board, &move_) {
                            legal_moves.push(move_);
                        }
                    }
                }
            } else if piece.0 == PieceType::Knight {
                for dest in KnightHopIter::new(src) {
                    let move_ = Move::Normal { src, dest };
                    if is_move_legal(board, &move_) {
                        legal_moves.push(move_);
                    }
                }
            } else if piece.0 == PieceType::King {
                for dest in DirIter::all().filter_map(|dir| pos_plus(src, dir)) {
                    let move_ = Move::Normal { src, dest };
                    if is_move_legal(board, &move_) {
                        legal_moves.push(move_);
                    }
                }
            } else {
                for dir in match piece.0 {
                    PieceType::Rook => DirIter::rook(),
                    PieceType::Bishop => DirIter::bishop(),
                    PieceType::Queen => DirIter::all(),
                    _ => unreachable!(),
                } {
                    for dest in RayIter::new(src, dir) {
                        let move_ = Move::Normal { src, dest };
                        if is_move_legal(board, &move_) {
                            legal_moves.push(move_);
                        }
                    }
                }
            }
        }
    }
    if is_move_legal(board, &Move::CastleKingside {}) {
        legal_moves.push(Move::CastleKingside {});
    }
    if is_move_legal(board, &Move::CastleQueenside {}) {
        legal_moves.push(Move::CastleQueenside {});
    }
    legal_moves
}

pub fn get_gamestate(board: &Board) -> GameState {
    let legal_moves = get_legal_moves(board);
    if legal_moves.is_empty() {
        if is_king_in_check(board) {
            GameState::Mated(board.active_player)
        } else {
            GameState::Stalemate
        }
    } else {
        GameState::InProgress
    }
}
