#[cfg(test)]
mod tests;

use crate::chess::{Board, Move};

use super::{Color, File, GameState, Position, Rank};

struct SlideIter {
    current: Position,
    dest: Position,
    step: (i8, i8),
}

fn pos_plus(pos: &Position, step: (i8, i8)) -> Position {
    Position(File::from_i8(pos.0 as i8 + step.0).unwrap(), Rank::from_i8(pos.1 as i8 + step.1).unwrap())
}

impl SlideIter {
    // iterator includes all positions between src and dest, excluding src and dest
    fn new(src: Position, dest: Position) -> SlideIter {
        let step = (
            ((dest.0 as i8) - (src.0 as i8)).signum(),
            ((dest.1 as i8) - (src.1 as i8)).signum(),
        );
        let current = pos_plus(&src, step);
        SlideIter { current, dest, step }
    }
}

impl Iterator for SlideIter {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {

        if (self.current != self.dest) {
            let curr = self.current;
            self.current = pos_plus(&self.current, self.step);;
            Some(curr)
        } else {
            None
        }
    }
}

fn check_move_consistent_with_piece_type(src: &Position, dest: &Position, player: Color) -> Result<(), String> {

    unimplemented!("is_move_consistent_with_piece_type")
}

fn handle_normal_move(board: &Board, src: &Position, dest: &Position) -> Result<Board, String> {
    let src_piece = match board.get_piece_at(src) {
        Some(piece) => piece,
        None => return Err("No piece at from".to_string()),
    };

    if (src_piece.1 != board.active_player) {
        return Err("Tried to move opponent's piece".to_string());
    }

    // if dest is occupied by a piece of the same color, return error
    board.get_piece_at(dest).map_or(Ok(()), |dest_piece| {
        if dest_piece.1 == board.active_player {
            Err("Tried to move to a square occupied by a piece of the same color".to_string())
        } else {
            Ok(())
        }
    })?;

    // if movement pattern is not consistent with piece type, return error
    
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
