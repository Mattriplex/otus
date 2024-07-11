use crate::board::{model_utils::ColorProps, models::{Color, File, Piece, PieceType, Rank, Square}, Board};

use super::movement_patterns::{pos_plus, DirIter, KnightHopIter, RayIter, SlideIter, SquareIter};


pub struct PlayerPieceIter<'a> {
    board: &'a Board,
    player: Color,
    square_iter: SquareIter,
}

impl<'a> PlayerPieceIter<'a> {
    pub fn new(board: &'a Board, player: Color) -> PlayerPieceIter<'a> {
        PlayerPieceIter {
            board,
            player,
            square_iter: SquareIter::new(),
        }
    }
}

// Iterates through a player's pieces
impl<'a> Iterator for PlayerPieceIter<'a> {
    type Item = (PieceType, Square);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(square) = self.square_iter.next() {
                if let Some(Piece(piece, owner)) = self.board.get_piece_at(square) {
                    if owner == self.player {
                        return Some((piece, square));
                    }
                }
            } else {
                return None;
            }
        }
    }
}

// Is square under attack from opponent of active player
pub fn is_square_attacked(board: &Board, target: Square) -> bool {
    let active_player = board.active_player;
    let pawn_dir = match board.active_player {
        Color::White => 1,
        Color::Black => -1,
    };
    let pawn_attack_dirs = [(1, pawn_dir), (-1, pawn_dir)];

    // diagonal moves
    for dir in DirIter::bishop() {
        let pos = match pos_plus(target, dir) {
            Some(pos) => pos,
            None => continue,
        };
        if let Some(Piece(piece, owner)) = board.get_piece_at(pos) {
            if owner == active_player {
                continue; //this direction is safe, attacks blocked by friendly piece
            }
            if pawn_attack_dirs.contains(&dir) && piece == PieceType::Pawn {
                return true;
            }
            match piece {
                PieceType::Bishop => return true,
                PieceType::Queen => return true,
                PieceType::King => return true,
                _ => continue, // enemy piece blocks diagonal attacks
            }
            // cast ray to detect distant attackers
        };
        for pos in RayIter::new(pos, dir) {
            if let Some(Piece(piece, owner)) = board.get_piece_at(pos) {
                if owner == active_player {
                    break; //attacks blocked by friendly piece
                }
                match piece {
                    PieceType::Bishop => return true,
                    PieceType::Queen => return true,
                    _ => break, // enemy piece blocks sliding attacks
                }
            }
        }
    }

    // horizontal and vertical moves
    for dir in DirIter::rook() {
        let pos = match pos_plus(target, dir) {
            Some(pos) => pos,
            None => continue,
        };
        if let Some(Piece(piece, owner)) = board.get_piece_at(pos) {
            if owner == active_player {
                continue; //this direction is safe, attacks blocked by friendly piece
            }
            match piece {
                PieceType::Rook => return true,
                PieceType::Queen => return true,
                PieceType::King => return true,
                _ => continue, // enemy piece blocks horizontal and vertical attacks
            }
            // cast ray to detect distant attackers
        };
        for pos in RayIter::new(pos, dir) {
            if let Some(Piece(piece, owner)) = board.get_piece_at(pos) {
                if owner == active_player {
                    break; //attacks blocked by friendly piece
                }
                match piece {
                    PieceType::Rook => return true,
                    PieceType::Queen => return true,
                    _ => break, // enemy piece blocks sliding attacks
                }
            }
        }
    }

    // knight moves
    for pos in KnightHopIter::new(target) {
        if let Some(Piece(PieceType::Knight, owner)) = board.get_piece_at(pos) {
            if owner != active_player {
                return true;
            }
        }
    }
    false
}


/** To be used with bishop, rook, queen
* returns true if squares between src and dest are free of pieces (exclusive)
*/
pub fn is_sliding_path_free(board: &Board, src: Square, dest: Square) -> bool {
    let slide_iter = SlideIter::new(src, dest);
    for pos in slide_iter {
        if board.get_piece_at(pos).is_some() {
            return false;
        }
    }
    true
}

pub fn seek_king(board: &Board, color: Color) -> Square {
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

/// new_board: Move is already carried out, but active player is not switched yet
pub fn is_king_in_check(new_board: &Board) -> bool {
    let king_pos = seek_king(new_board, new_board.active_player);
    is_square_attacked(new_board, king_pos)
}

/// this function does not check if the pawn belongs to the active player, handle_normal_move does that
pub fn is_promotion_move(board: &Board, src: Square, dest: Square) -> bool {
    match board.get_piece_at(src) {
        Some(Piece(PieceType::Pawn, Color::White)) => dest.1 == Rank::_8,
        Some(Piece(PieceType::Pawn, Color::Black)) => dest.1 == Rank::_1,
        _ => false,
    }
}