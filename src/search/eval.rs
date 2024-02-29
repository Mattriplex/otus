use std::cmp::min;

use crate::board::{
    model_utils::ColorProps,
    models::{Color, File, Piece, PieceType, Square},
    move_checking::{
        is_king_in_check, seek_king,
        square_utils::{pos_plus, SquareIter},
    },
    Board,
};

pub fn get_material_eval(board: &Board) -> f32 {
    let mut material_balance = 0.0;
    for square in SquareIter::new() {
        if let Some(Piece(piece, owner)) = board.get_piece_at(square) {
            let value = match piece {
                PieceType::Pawn => 1.0,
                PieceType::Knight => 3.0,
                PieceType::Bishop => 3.0,
                PieceType::Rook => 5.0,
                PieceType::Queen => 9.0,
                PieceType::King => continue,
            };
            material_balance += if owner == board.active_player {
                value
            } else {
                -value
            };
        }
    }
    material_balance
}

fn get_knight_value(square: Square) -> f32 {
    let dx = min(square.0 as i32, 7 - square.0 as i32);
    let dy = min(square.0 as i32, 7 - square.0 as i32);
    let num_jumps = match (dx, dy) {
        (0, 0) => 2,
        (1, 0) | (0, 1) => 3,
        (1, 1) | (0, 2) | (2, 0) | (0, 3) | (3, 0) => 4,
        (1, 2) | (2, 1) | (1, 3) | (3, 1) => 6,
        _ => 8,
    };
    return 250.0 + 10.0 * num_jumps as f32;
}

fn middlegame_bonuses(board: &Board) -> f32 {
    let active_player = board.active_player;
    // king safety bonus
    let mut score = 0.0;
    let king_sq = seek_king(board, active_player);
    // penalize king in center
    match king_sq.0 {
        File::E | File::D => score -= 20.0,
        File::F => score -= 10.0,
        _ => score += 20.0,
    }
    // pawn shield bonus (king on back rank, at least 2 pawns in front)
    if king_sq.1 == board.active_player.home_rank() {
        let mut pawn_shield = 0;
        for pos in [(-1, 0), (0, 0), (1, 0)]
            .iter()
            .filter_map(|dir| pos_plus(Square(king_sq.0, active_player.pawn_start_rank()), *dir))
        {
            if let Some(Piece(PieceType::Pawn, owner)) = board.get_piece_at(pos) {
                if owner == board.active_player {
                    pawn_shield += 1;
                }
            }
        }
        if pawn_shield == 1 {
            score += 30.0;
        }
        if pawn_shield >= 2 {
            score += 100.0;
        }
    }
    score
}

fn endgame_bonuses(board: &Board) -> f32 {
    // if endgame -> enemy king near edge bonus
    let mut score = 0.0;
    let king_sq = seek_king(board, board.active_player.opponent());
    // distance to edge
    let dx = min(king_sq.0 as i32, 7 - king_sq.0 as i32);
    let dy = min(king_sq.0 as i32, 7 - king_sq.0 as i32);
    score += (3.0 - min(dx, dy) as f32) * 10.0;
    score
}

fn get_pawn_value(square: Square, color: Color) -> f32 {
    let rank = square.1;
    let home_rank = color.pawn_start_rank();
    let dist_bonus = match (rank as i32 - home_rank as i32).abs() {
        0 => 0.0,
        1 => 10.0,
        2 => 20.0,
        3 => 30.0,
        4 => 40.0,
        _ => 220.0, // about to promote, extremely valuable
    };
    // middle pawns are more valuable
    let file = square.0;
    let file_bonus = match file {
        File::A | File::H => 0.0,
        File::B | File::G => 5.0,
        File::C | File::F => 10.0,
        File::D | File::E => 20.0,
    };
    100.0 + dist_bonus + file_bonus
}

// score in centipawns
pub fn smart_eval(board: &Board) -> f32 {
    let mut score = 0.0;
    // check penalty
    let in_check = is_king_in_check(board);
    if in_check {
        score -= 30.0;
    }
    let mut my_material = 0.0;
    let mut opp_material = 0.0;
    for sq in SquareIter::new() {
        if let Some(Piece(piece, owner)) = board.get_piece_at(sq) {
            let value = match piece {
                PieceType::Pawn => get_pawn_value(sq, owner),
                PieceType::Knight => get_knight_value(sq),
                PieceType::Bishop => 310.0,
                PieceType::Rook => 500.0,
                PieceType::Queen => 900.0,
                PieceType::King => 0.0,
            };
            if owner == board.active_player {
                my_material += value;
            } else {
                opp_material += value;
            }
        }
    }
    // TODO score pawns differently depending on phase
    score += my_material - opp_material;
    let is_endgame = my_material + opp_material < 3300.0;
    // king safety bonus
    score += if is_endgame {
        endgame_bonuses(board)
    } else {
        middlegame_bonuses(board)
    };
    // open file bonus

    score
}

// TODO: figure out why knight into corner is best move in N7/7p/4k1p1/p3pp2/1b4Pr/5P2/6KP/1R6 b - - 1 37
