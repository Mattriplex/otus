#[cfg(test)]
mod tests;

use crate::chess::{Board, Move};

use super::{
    Color, File, GameState, Opponent, Piece, PieceType, Position, PromotionPieceType, Rank,
};

struct SlideIter {
    current: Position,
    dest: Position,
    step: (i8, i8),
}

fn pos_plus(pos: &Position, step: (i8, i8)) -> Option<Position> {
    let new_file = match File::from_i8(pos.0 as i8 + step.0) {
        Some(file) => file,
        None => return None,
    };
    let new_rank = match Rank::from_i8(pos.1 as i8 + step.1) {
        Some(rank) => rank,
        None => return None,
    };
    Some(Position(new_file, new_rank))
}

fn pos_minus(dest: &Position, src: &Position) -> (i8, i8) {
    (
        (dest.0 as i8) - (src.0 as i8),
        (dest.1 as i8) - (src.1 as i8),
    )
}

const ROOK_DIRS: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

const BISHOP_DIRS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

const KNIGHT_HOPS: [(i8, i8); 8] = [
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, -1),
    (-2, 1),
    (-1, 2),
];

struct KnightHopIter {
    current: usize,
    positions: [Option<Position>; 8],
}

impl KnightHopIter {
    fn new(origin: &Position) -> KnightHopIter {
        let mut p_idx = 0;
        let mut positions = [None; 8];
        for hop in KNIGHT_HOPS.iter() {
            if let Some(pos) = pos_plus(origin, *hop) {
                positions[p_idx] = Some(pos);
                p_idx += 1;
            }
        }
        KnightHopIter {
            current: 0,
            positions,
        }
    }
}

impl Iterator for KnightHopIter {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < 8 {
            let pos = self.positions[self.current];
            self.current += 1;
            pos
        } else {
            None
        }
    }
}

struct DirIter {
    current: usize,
    dirs: &'static [(i8, i8)],
}

impl DirIter {
    fn new(dirs: &'static [(i8, i8)]) -> DirIter {
        DirIter { current: 0, dirs }
    }
}

impl Iterator for DirIter {
    type Item = (i8, i8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.dirs.len() {
            let dir = self.dirs[self.current];
            self.current += 1;
            Some(dir)
        } else {
            None
        }
    }
}

struct RayIter {
    base: Position,
    dir: (i8, i8),
    current: Position,
}

impl RayIter {
    fn new(base: Position, dir: (i8, i8)) -> RayIter {
        RayIter {
            base,
            dir,
            current: base,
        }
    }
}

impl Iterator for RayIter {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        match pos_plus(&self.current, self.dir) {
            Some(pos) => {
                self.current = pos;
                Some(pos)
            }
            None => None,
        }
    }
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
        let current = pos_plus(&src, step).unwrap();
        SlideIter {
            current,
            dest,
            step,
        }
    }
}

impl Iterator for SlideIter {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current != self.dest {
            let curr = self.current;
            self.current =
                pos_plus(&self.current, self.step).expect("SlideIter::next: step out of bounds");
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
        Color::White => (x.abs() <= 1 && y == 1) || (x == 0 && y == 2 && src.1 == Rank::_2),
        Color::Black => (x.abs() <= 1 && y == -1) || (x == 0 && y == -2 && src.1 == Rank::_7),
    }
}

fn check_piece_move_consistency(
    src: &Position,
    dest: &Position,
    piece: PieceType,
    player: Color,
) -> Result<(), String> {
    let consistent = match piece {
        PieceType::Queen => is_rook_move(src, dest) || is_bishop_move(src, dest),
        PieceType::Rook => is_rook_move(src, dest),
        PieceType::Bishop => is_bishop_move(src, dest),
        PieceType::Knight => is_knight_move(src, dest),
        PieceType::King => is_king_move(src, dest),
        PieceType::Pawn => is_pawn_move(src, dest, player),
    };
    if consistent {
        return Ok(());
    } else {
        return Err(format!("{} cannot move this way", piece));
    };
}

fn check_move_blocked(
    piece: PieceType,
    src: &Position,
    dest: &Position,
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
    }
    .expect("En passant target neighbour out of bounds");
    new_board.clear_square(captured_pawn_pos);
    new_board.en_passant_target = None;
}

fn seek_king(board: &Board, color: Color) -> Option<Position> {
    for file in 0..8 {
        for rank in 0..8 {
            let pos = Position(File::from_i8(file).unwrap(), Rank::from_i8(rank).unwrap());
            if let Some(Piece(PieceType::King, c)) = board.get_piece_at(&pos) {
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

    // cast rays from king to check for threats
    for dir in DirIter::new(&ROOK_DIRS) {
        for pos in RayIter::new(king_pos, dir) {
            if let Some(Piece(piece, color)) = new_board.get_piece_at(&pos) {
                if color == new_board.active_player {
                    break;
                }
                if piece == PieceType::Rook || piece == PieceType::Queen {
                    return true;
                }
            }
        }
    }
    for dir in DirIter::new(&BISHOP_DIRS) {
        for pos in RayIter::new(king_pos, dir) {
            if let Some(Piece(piece, color)) = new_board.get_piece_at(&pos) {
                if color == new_board.active_player {
                    break;
                }
                if piece == PieceType::Bishop || piece == PieceType::Queen {
                    return true;
                }
            }
        }
    }

    // check knight moves
    for pos in KnightHopIter::new(&king_pos) {
        if let Some(Piece(PieceType::Knight, color)) = new_board.get_piece_at(&pos) {
            if color != new_board.active_player {
                return true;
            }
        }
    }

    return false;
}

fn update_castling_rights(new_board: &mut Board, src: &Position, dest: &Position) {
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
    if dest.clone() == Position(File::A, opp_home_rank) {
        new_board.revoke_queenside_castling(active_player.opponent())
    } else if dest.clone() == Position(File::H, opp_home_rank) {
        new_board.revoke_kingside_castling(active_player.opponent())
    }
    // moving the king removes castling rights
    let (can_kingside_castle, can_queenside_castle) = (
        new_board.can_castle_kingside(active_player),
        new_board.can_castle_queenside(active_player),
    );
    if (can_kingside_castle || can_queenside_castle) && src.clone() == Position(File::E, home_rank)
    {
        new_board.revoke_kingside_castling(active_player);
        new_board.revoke_queenside_castling(active_player);
    }
    // moving a rook removes castling rights
    if can_kingside_castle && src.clone() == Position(File::H, home_rank) {
        new_board.revoke_kingside_castling(active_player);
    }
    if can_queenside_castle && src.clone() == Position(File::A, home_rank) {
        new_board.revoke_queenside_castling(active_player);
    }
}

//TODO: Double pawn move sets en passant target
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

    handle_en_passant_move(&mut new_board);
    // check if king in check
    if is_king_in_check(&new_board) {
        return Err("Move would leave king in check".to_string());
    }

    update_castling_rights(&mut new_board, src, dest);
    new_board.active_player = board.active_player.opponent();

    Ok(new_board)
}

// this function does not check if the pawn belongs to the active player, handle_normal_move does that
fn is_promotion_move(board: &Board, src: &Position, dest: &Position) -> bool {
    match board.get_piece_at(src) {
        Some(Piece(PieceType::Pawn, Color::White)) => dest.1 == Rank::_8,
        Some(Piece(PieceType::Pawn, Color::Black)) => dest.1 == Rank::_1,
        _ => false,
    }
}

fn handle_promotion_move(board: &Board, move_: &Move) -> Result<Board, String> {
    let (src, dest, promotion) = match move_ {
        Move::Promotion {
            from,
            to,
            promotion,
        } => (from, to, promotion),
        _ => unreachable!(),
    };

    if !is_promotion_move(board, src, dest) {
        return Err("Not a promotion move".to_string());
    }

    let mut new_board = handle_normal_move(board, src, dest)?;

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

fn check_and_handle_normal_move(
    board: &Board,
    src: &Position,
    dest: &Position,
) -> Result<Board, String> {
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
        Position(File::E, home_rank),
        Position(File::F, home_rank),
        Position(File::G, home_rank),
        Position(File::H, home_rank),
    );
    let mut new_board = board.clone();
    new_board.clear_square(king_pos);
    // F square must be empty and not under attack
    if let Some(_) = board.get_piece_at(&f_square) {
        return Err("Cannot castle through occupied square".to_string());
    }
    new_board.set_piece_at(f_square, Piece(PieceType::King, board.active_player));
    if is_king_in_check(&new_board) {
        return Err("Cannot castle through check".to_string());
    }
    new_board.clear_square(f_square);
    // G square must be empty and not under attack
    if let Some(_) = board.get_piece_at(&g_square) {
        return Err("Cannot castle through occupied square".to_string());
    }
    new_board.set_piece_at(g_square, Piece(PieceType::King, board.active_player));
    if is_king_in_check(&new_board) {
        return Err("Cannot castle into check".to_string());
    }
    new_board.clear_square(rook_pos);
    new_board.set_piece_at(f_square, Piece(PieceType::Rook, board.active_player));
    new_board.active_player = board.active_player.opponent();
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
        Position(File::E, home_rank),
        Position(File::D, home_rank),
        Position(File::C, home_rank),
        Position(File::B, home_rank),
        Position(File::A, home_rank),
    );
    let mut new_board = board.clone();
    new_board.clear_square(king_pos);
    // D square must be empty and not under attack
    if let Some(_) = board.get_piece_at(&d_square) {
        return Err("Cannot castle through occupied square".to_string());
    }
    new_board.set_piece_at(d_square, Piece(PieceType::King, board.active_player));
    if is_king_in_check(&new_board) {
        return Err("Cannot castle through check".to_string());
    }
    new_board.clear_square(d_square);
    // C square must be empty and not under attack
    if let Some(_) = board.get_piece_at(&c_square) {
        return Err("Cannot castle through occupied square".to_string());
    }
    new_board.set_piece_at(c_square, Piece(PieceType::King, board.active_player));
    if is_king_in_check(&new_board) {
        return Err("Cannot castle into check".to_string());
    }
    if let Some(_) = board.get_piece_at(&b_square) {
        return Err("Cannot castle through occupied square".to_string());
    }
    new_board.clear_square(rook_pos);
    new_board.set_piece_at(d_square, Piece(PieceType::Rook, board.active_player));
    new_board.active_player = board.active_player.opponent();
    Ok(new_board)
}

// TODO: switch active player
pub fn apply_move(board: &Board, move_: &Move) -> Result<Board, String> {
    match move_ {
        Move::Normal { from, to } => check_and_handle_normal_move(board, from, to),
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

fn get_legal_moves(board: &Board) -> Vec<Move> {
    unimplemented!("get_legal_moves")
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
