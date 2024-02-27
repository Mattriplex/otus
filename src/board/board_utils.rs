use super::{
    models::{Color, Piece, PieceType, Square},
    move_checking::square_utils::{pos_plus, DirIter, KnightHopIter, RayIter, SquareIter},
    Board,
};

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
