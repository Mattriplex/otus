use std::fmt::{self, Display};

use crate::{board::models::Piece, players::ChessPlayer};

use self::{
    models::{Color, File, GameState, Move, PieceType, PromotionPieceType, Rank, Square},
    move_checking::{
        is_king_in_check, is_move_legal,
        square_utils::{pos_plus, DirIter, KnightHopIter, RayIter},
    },
};

pub mod move_checking;

pub mod model_utils;
pub mod models;

#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
    pub active_player: Color,
    castling_rights: u8, // KQkq
    en_passant_target: Option<Square>,
}

impl Board {
    pub fn empty() -> Board {
        Board {
            squares: [[None; 8]; 8],
            active_player: Color::White,
            castling_rights: 0,
            en_passant_target: None,
        }
    }

    pub fn default() -> Board {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    fn get_piece(&self, file: File, rank: Rank) -> Option<Piece> {
        self.squares[rank as usize][file as usize]
    }

    fn get_piece_at(&self, pos: Square) -> Option<Piece> {
        self.get_piece(pos.0, pos.1)
    }

    fn set_piece(&mut self, file: File, rank: Rank, piece: Piece) {
        self.squares[rank as usize][file as usize] = Some(piece);
    }

    fn set_piece_at(&mut self, pos: Square, piece: Piece) {
        self.squares[pos.1 as usize][pos.0 as usize] = Some(piece);
    }

    fn clear_square(&mut self, pos: Square) {
        self.squares[pos.1 as usize][pos.0 as usize] = None;
    }

    fn squares_from_fen(fen_squares: &str) -> Result<[[Option<Piece>; 8]; 8], String> {
        let mut squares: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
        let mut rank: usize = 7;
        let mut file: usize = 0;
        for c in fen_squares.chars() {
            match c {
                '/' => {
                    if file != 8 {
                        return Err(format!("Rank {} contains too few squares", rank + 1));
                    }
                    if rank == 0 {
                        return Err("Expected end of fen, but got more squares".to_string());
                    }
                    rank -= 1;
                    file = 0;
                }
                '1'..='8' => {
                    let empty_squares = c.to_digit(10).unwrap() as usize;
                    if file + empty_squares > 8 {
                        return Err(format!("Rank {} contains too many empty squares", rank + 1));
                    }
                    file += empty_squares;
                }
                _ => {
                    let piece = match c {
                        'p' => Piece(PieceType::Pawn, Color::Black),
                        'n' => Piece(PieceType::Knight, Color::Black),
                        'b' => Piece(PieceType::Bishop, Color::Black),
                        'r' => Piece(PieceType::Rook, Color::Black),
                        'q' => Piece(PieceType::Queen, Color::Black),
                        'k' => Piece(PieceType::King, Color::Black),
                        'P' => Piece(PieceType::Pawn, Color::White),
                        'N' => Piece(PieceType::Knight, Color::White),
                        'B' => Piece(PieceType::Bishop, Color::White),
                        'R' => Piece(PieceType::Rook, Color::White),
                        'Q' => Piece(PieceType::Queen, Color::White),
                        'K' => Piece(PieceType::King, Color::White),
                        _ => return Err(format!("Invalid character in fen: {}", c)),
                    };
                    if file >= 8 {
                        return Err(format!("Rank {} contains too many pieces", rank + 1));
                    }
                    squares[rank][file] = Some(piece);
                    file += 1;
                }
            }
        }
        if rank > 0 {
            return Err(format!("Expected {} more ranks", rank));
        }
        Ok(squares)
    }

    fn decode_fen_castling_rights(castling_rights: &str) -> Result<u8, String> {
        let mut result = 0;
        for c in castling_rights.chars() {
            match c {
                'K' => result |= 0b1000,
                'Q' => result |= 0b0100,
                'k' => result |= 0b0010,
                'q' => result |= 0b0001,
                '-' => (),
                _ => return Err(format!("Invalid character in castling rights: {}", c)),
            }
        }
        Ok(result)
    }

    pub fn from_fen(fen: &str) -> Result<Board, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 6 {
            return Err("Expected 6 parts in fen".to_string());
        }
        let squares = Self::squares_from_fen(parts[0])?;
        let active_player = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(format!("Invalid active player: {}", parts[1])),
        };
        let castling_rights = Self::decode_fen_castling_rights(parts[2])?;
        let en_passant_target = match parts[3] {
            "-" => None,
            s => Some(Square::from_string(s)?),
        };
        Ok(Board {
            squares,
            active_player,
            castling_rights,
            en_passant_target,
        })
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        for rank in (0..8).rev() {
            let mut empty_squares = 0;
            for file in 0..8 {
                match self.squares[rank][file] {
                    Some(piece) => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        let p_char = match piece {
                            Piece(PieceType::Pawn, Color::White) => 'P',
                            Piece(PieceType::Knight, Color::White) => 'N',
                            Piece(PieceType::Bishop, Color::White) => 'B',
                            Piece(PieceType::Rook, Color::White) => 'R',
                            Piece(PieceType::Queen, Color::White) => 'Q',
                            Piece(PieceType::King, Color::White) => 'K',
                            Piece(PieceType::Pawn, Color::Black) => 'p',
                            Piece(PieceType::Knight, Color::Black) => 'n',
                            Piece(PieceType::Bishop, Color::Black) => 'b',
                            Piece(PieceType::Rook, Color::Black) => 'r',
                            Piece(PieceType::Queen, Color::Black) => 'q',
                            Piece(PieceType::King, Color::Black) => 'k',
                        };
                        fen.push(p_char);
                    }
                    None => empty_squares += 1,
                }
            }
            if empty_squares > 0 {
                fen.push_str(&empty_squares.to_string());
            }
            if rank > 0 {
                fen.push('/');
            }
        }
        fen.push(' ');
        fen.push_str(&match self.active_player {
            Color::White => "w",
            Color::Black => "b",
        });
        fen.push(' ');
        if self.castling_rights == 0 {
            fen.push('-');
        } else {
            if self.castling_rights & 0b1000 != 0 {
                fen.push('K');
            }
            if self.castling_rights & 0b0100 != 0 {
                fen.push('Q');
            }
            if self.castling_rights & 0b0010 != 0 {
                fen.push('k');
            }
            if self.castling_rights & 0b0001 != 0 {
                fen.push('q');
            }
        }
        fen.push(' ');
        match self.en_passant_target {
            Some(p) => fen.push_str(&p.to_string()),
            None => fen.push('-'),
        }
        fen.push_str(" 0 1");
        fen
    }

    fn can_castle_kingside(&self, color: Color) -> bool {
        self.castling_rights
            & match color {
                Color::White => 0b1000,
                Color::Black => 0b0010,
            }
            != 0
    }

    fn can_castle_queenside(&self, color: Color) -> bool {
        self.castling_rights
            & match color {
                Color::White => 0b0100,
                Color::Black => 0b0001,
            }
            != 0
    }

    fn revoke_kingside_castling(&mut self, color: Color) {
        self.castling_rights &= match color {
            Color::White => 0b0111,
            Color::Black => 0b1101,
        }
    }

    fn revoke_queenside_castling(&mut self, color: Color) {
        self.castling_rights &= match color {
            Color::White => 0b1011,
            Color::Black => 0b1110,
        }
    }

    fn fmt_rank(&self, f: &mut fmt::Formatter, rank: usize) -> fmt::Result {
        for file in 0..8 {
            match self.squares[rank][file] {
                Some(p) => p.fmt(f),
                None => write!(f, "."),
            }?;
        }
        Ok(())
    }

    // TODO clean up
    // step 1: get list of possible moves (depends on piece)
    // step 2: filter illegal moves
    pub fn get_legal_moves(&self) -> Vec<Move> {
        let mut legal_moves = Vec::new();
        let (opp_home_rank, pawn_dir) = match self.active_player {
            Color::White => (Rank::_8, 1),
            Color::Black => (Rank::_1, -1),
        };
        for file in 0..8 {
            for rank in 0..8 {
                let src = Square(File::from_i8(file).unwrap(), Rank::from_i8(rank).unwrap());
                let piece = match self.get_piece_at(src) {
                    Some(piece) => piece,
                    None => continue,
                };
                if piece.1 != self.active_player {
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
                            if is_move_legal(self, &move_) {
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
                            if is_move_legal(self, &move_) {
                                legal_moves.push(move_);
                            }
                        }
                    }
                } else if piece.0 == PieceType::Knight {
                    for dest in KnightHopIter::new(src) {
                        let move_ = Move::Normal { src, dest };
                        if is_move_legal(self, &move_) {
                            legal_moves.push(move_);
                        }
                    }
                } else if piece.0 == PieceType::King {
                    for dest in DirIter::all().filter_map(|dir| pos_plus(src, dir)) {
                        let move_ = Move::Normal { src, dest };
                        if is_move_legal(self, &move_) {
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
                            if is_move_legal(self, &move_) {
                                legal_moves.push(move_);
                            }
                        }
                    }
                }
            }
        }
        if is_move_legal(self, &Move::CastleKingside {}) {
            legal_moves.push(Move::CastleKingside {});
        }
        if is_move_legal(self, &Move::CastleQueenside {}) {
            legal_moves.push(Move::CastleQueenside {});
        }
        legal_moves
    }

    pub fn get_gamestate(&self) -> GameState {
        let legal_moves = self.get_legal_moves();
        if legal_moves.is_empty() {
            if is_king_in_check(self) {
                GameState::Mated(self.active_player)
            } else {
                GameState::Stalemate
            }
        } else {
            GameState::InProgress
        }
    }
}

pub fn run_game(white_player: &dyn ChessPlayer, black_player: &dyn ChessPlayer) -> GameState {
    let mut board = Board::default();
    loop {
        let m = match board.active_player {
            Color::White => white_player.make_move(&board),
            Color::Black => black_player.make_move(&board),
        };
        match move_checking::apply_move(&board, &m) {
            Ok(new_board) => {
                board = new_board;
                match board.get_gamestate() {
                    GameState::InProgress => (),
                    gs => return gs,
                }
            }
            // Illegal move, game is forfeit
            Err(_) => return GameState::Mated(board.active_player),
        }
    }
}