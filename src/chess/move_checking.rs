#[cfg(test)]
mod tests;

use crate::chess::{Board, Move};

use super::{Color, File, GameState, Piece, PieceType, Position, Rank};

struct SlideIter {
    current: Position,
    dest: Position,
    step: (i8, i8),
}

fn pos_plus(pos: &Position, step: (i8, i8)) -> Position {
    Position(File::from_i8(pos.0 as i8 + step.0).unwrap(), Rank::from_i8(pos.1 as i8 + step.1).unwrap())
}

fn pos_minus(dest: &Position, src: &Position) -> (i8, i8) {
    ((dest.0 as i8) - (src.0 as i8), (dest.1 as i8) - (src.1 as i8))
}

impl SlideIter {
    // iterator includes all positions between src and dest, excluding src and dest
    fn new(src: Position, dest: Position) -> SlideIter {
        let step = (
            ((dest.0 as i8) - (src.0 as i8)).signum(),
            ((dest.1 as i8) - (src.1 as i8)).signum(),
        );
        if !is_rook_move(&src, &dest) && !is_bishop_move(&src, &dest) {
            panic!("SlideIter::new called with non-sliding move");
        }
        let current = pos_plus(&src, step);
        SlideIter { current, dest, step }
    }
}

impl Iterator for SlideIter {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {

        if self.current != self.dest {
            let curr = self.current;
            self.current = pos_plus(&self.current, self.step);
            Some(curr)
        } else {
            None
        }
    }
}

fn is_rook_move(src: &Position, dest: &Position) -> bool {
    let (x, y) = pos_minus(dest, src);
    x == 0 || y == 0
}

fn is_bishop_move(src: &Position, dest: &Position) -> bool {
    let (x, y) = pos_minus(dest, src);
    x.abs() == y.abs()
}

fn is_knight_move(src: &Position, dest: &Position) -> bool {
    let (x, y) = pos_minus(dest, src);
    (x.abs() == 2 && y.abs() == 1) || (x.abs() == 1 && y.abs() == 2)
}

fn is_king_move(src: &Position, dest: &Position) -> bool {
    let (x, y) = pos_minus(dest, src);
    x.abs() <= 1 && y.abs() <= 1
}

fn is_pawn_move(src: &Position, dest: &Position, player: Color) -> bool {
    let (x, y) = pos_minus(dest, src);
    match player {
        Color::White => (x.abs() <= 1 && y == 1) || (x == 0 && y == 2 && src.1 == 2)
        Color::Black => (x.abs() <= 1 && y == -1) || (x == 0 && y == -2 && src.1 == 7)
    }
}

fn check_piece_move_consistency(src: &Position, dest: &Position, piece: PieceType, player: Color) -> Result<(), String> {
    let consistent = match piece {
        PieceType::Queen => is_rook_move(src, dest) || is_bishop_move(src, dest),
        PieceType::Rook => is_rook_move(src, dest),
        PieceType::Bishop => is_bishop_move(src, dest),
        PieceType::Knight => is_knight_move(src, dest),
        PieceType::King => is_king_move(src, dest),
        PieceType::Pawn => is_pawn_move(src, dest, player),
    };
    if consistent { 
        return Ok(())
    } else { 
        return Err(format!("{} cannot move this way", piece)) 
    };
}

fn check_move_blocked(piece: PieceType, src: &Position, dest: &Position, board: &Board) -> Result<(), String> {
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
            if board.get_piece_at(dest).is_none() && board.en_passant_target != Some(*dest) {
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
    let slide_iter = SlideIter::new(*src, *dest);
    for pos in slide_iter {
        if board.get_piece_at(&pos).is_some() {
            return Err("Sliding move is blocked".to_string());
        }
    }
    Ok(())
}

// Precondition: pawn move already carried out
fn handle_en_passant_move(new_board: &mut Board) {
    // if en passant was played, remove the captured pawn
    let en_passant_target = match new_board.en_passant_target {
        Some(pos) => pos,
        None => return,
    };

    if new_board.get_piece_at(&en_passant_target) == None {
        return; // if en passant was played, it would have the active player's pawn on it
    }
    let captured_pawn_pos = match new_board.active_player {
        Color::White => pos_plus(&en_passant_target, (0, -1)),
        Color::Black => pos_plus(&en_passant_target, (0, 1)),
    };
    new_board.clear_square(captured_pawn_pos);
    new_board.en_passant_target = None;
}

fn handle_normal_move(board: &Board, src: &Position, dest: &Position) -> Result<Board, String> {
    let src_piece = match board.get_piece_at(src) {
        Some(piece) => piece,
        None => return Err("No piece at move origin".to_string()),
    };

    if src_piece.1 != board.active_player {
        return Err("Tried to move opponent's piece".to_string());
    }

    check_piece_move_consistency(src, dest, src_piece.0, board.active_player)?;
    check_move_blocked(src_piece.0, src, dest, board)?;

    // carry out move
    let mut new_board = board.clone();
    new_board.set_piece_at(*dest, src_piece);
    new_board.clear_square(*src);
       
}

fn handle_castling_move(board: &Board, move_: &Move) -> Result<Board, String> {
    unimplemented!("handle_castling_move")
}

fn handle_promotion_move(board: &Board, move_: &Move) -> Result<Board, String> {
    unimplemented!("handle_promotion_move")
}

// TODO: switch active player
pub fn apply_move(board: &Board, move_: &Move) -> Result<Board, String> {
    match move_ {
        Move::Normal{from, to} => handle_normal_move(board, from, to),
        Move::CastleKingside{..} => handle_castling_move(board, move_),
        Move::CastleQueenside{..} => handle_castling_move(board, move_),
        Move::Promotion{..} => handle_promotion_move(board, move_),
    }
}

pub fn is_move_legal(board: &Board, move_: &Move) -> bool {
    match apply_move(board, move_) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn get_gamestate(board: &Board) -> GameState {
    unimplemented!("get_gamestate")
}
