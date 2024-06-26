use std::fmt::{self, Display};

use crate::board::models::Piece;

use self::{
    board_utils::PlayerPieceIter,
    model_utils::ColorProps,
    models::{
        Color, File, GameState, LegalMove, Move, PieceType, PromotionPieceType, Rank, Square,
    },
    move_checking::{
        can_castle_kingside, can_castle_queenside, get_legal_move_from_pseudolegal_move,
        is_king_in_check, is_move_legal,
        square_utils::{pos_plus, DirIter, KnightHopIter, RayIter},
    },
};

pub mod board_utils;
pub mod model_utils;
pub mod models;
pub mod move_checking;

#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
    pub active_player: Color,
    pub castling_rights: u8, // KQkq
    pub en_passant_target: Option<Square>,
}

impl Board {
    pub fn default() -> Board {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    fn get_piece(&self, file: File, rank: Rank) -> Option<Piece> {
        self.squares[rank as usize][file as usize]
    }

    pub fn get_piece_at(&self, pos: Square) -> Option<Piece> {
        self.get_piece(pos.0, pos.1)
    }

    fn set_piece_at(&mut self, pos: Square, piece: Piece) {
        self.squares[pos.1 as usize][pos.0 as usize] = Some(piece);
    }

    fn set_square(&mut self, pos: Square, piece: Option<Piece>) {
        self.squares[pos.1 as usize][pos.0 as usize] = piece;
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
        fen.push_str(match self.active_player {
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

    fn has_kingside_castling_rights(&self, color: Color) -> bool {
        self.castling_rights
            & match color {
                Color::White => 0b1000,
                Color::Black => 0b0010,
            }
            != 0
    }

    fn has_queenside_castling_rights(&self, color: Color) -> bool {
        self.castling_rights
            & match color {
                Color::White => 0b0100,
                Color::Black => 0b0001,
            }
            != 0
    }

    fn fmt_rank(&self, f: &mut fmt::Formatter, rank: usize) -> fmt::Result {
        for file in 0..8 {
            match self.squares[rank][file] {
                Some(p) => p.fmt(f),
                None => write!(f, "."),
            }?;
            write!(f, " ")?
        }
        Ok(())
    }

    fn try_add_promotion_moves(&self, src: Square, dest: Square, moves: &mut Vec<LegalMove>) {
        if let Some(LegalMove::Promotion { castle_mask, .. }) = get_legal_move_from_pseudolegal_move(
            self,
            &Move::Promotion {
                src,
                dest,
                promotion: PromotionPieceType::Queen,
            },
        ) {
            let dst_piece = self.get_piece_at(dest).map(|p| p.0);
            for promotion_piece in [
                PieceType::Queen,
                PieceType::Rook,
                PieceType::Bishop,
                PieceType::Knight,
            ] {
                moves.push(LegalMove::Promotion {
                    src,
                    dest,
                    promotion: promotion_piece,
                    castle_mask,
                    captured_piece: dst_piece,
                });
            }
        }
    }

    pub fn get_legal_moves(&self) -> Vec<LegalMove> {
        let mut legal_moves = Vec::new();
        let (forward, home_rank, opp_home_rank) = match self.active_player {
            Color::White => (1, Rank::_2, Rank::_7),
            Color::Black => (-1, Rank::_7, Rank::_2),
        };
        for (piece, src) in PlayerPieceIter::new(self, self.active_player) {
            let mk_ray = move |dir: (i8, i8)| RayIter::new(src, dir);
            match piece {
                PieceType::Queen => DirIter::all()
                    .flat_map(mk_ray)
                    .filter_map(|dest| {
                        get_legal_move_from_pseudolegal_move(self, &Move::Normal { src, dest })
                    })
                    .for_each(|m| legal_moves.push(m)),
                PieceType::Rook => DirIter::rook()
                    .flat_map(mk_ray)
                    .filter_map(|dest| {
                        get_legal_move_from_pseudolegal_move(self, &Move::Normal { src, dest })
                    })
                    .for_each(|m| legal_moves.push(m)),
                PieceType::Bishop => DirIter::bishop()
                    .flat_map(mk_ray)
                    .filter_map(|dest| {
                        get_legal_move_from_pseudolegal_move(self, &Move::Normal { src, dest })
                    })
                    .for_each(|m| legal_moves.push(m)),
                PieceType::Knight => KnightHopIter::new(src)
                    .filter_map(|dest| {
                        get_legal_move_from_pseudolegal_move(self, &Move::Normal { src, dest })
                    })
                    .for_each(|m| legal_moves.push(m)),
                PieceType::King => DirIter::all()
                    .flat_map(|dir| pos_plus(src, dir))
                    .filter_map(|dest| {
                        get_legal_move_from_pseudolegal_move(self, &Move::Normal { src, dest })
                    })
                    .for_each(|m| legal_moves.push(m)),
                PieceType::Pawn => {
                    if src.1 == opp_home_rank {
                        [(0, forward), (-1, forward), (1, forward)]
                            .iter()
                            .filter_map(move |step| pos_plus(src, *step))
                            .for_each(|dest| {
                                self.try_add_promotion_moves(src, dest, &mut legal_moves);
                            });
                    } else if home_rank == src.1 {
                        //include double pawn push
                        [(0, forward), (-1, forward), (1, forward), (0, 2 * forward)]
                            .iter()
                            .filter_map(move |step| pos_plus(src, *step))
                            .filter_map(|dest| {
                                get_legal_move_from_pseudolegal_move(
                                    self,
                                    &Move::Normal { src, dest },
                                )
                            })
                            .for_each(|m| legal_moves.push(m));
                    } else {
                        //exclude double pawn push
                        [(0, forward), (-1, forward), (1, forward)]
                            .iter()
                            .filter_map(move |step| pos_plus(src, *step))
                            .filter_map(|dest| {
                                get_legal_move_from_pseudolegal_move(
                                    self,
                                    &Move::Normal { src, dest },
                                )
                            })
                            .for_each(|m| legal_moves.push(m));
                    }
                }
            }
        }
        if can_castle_kingside(self) {
            legal_moves.push(LegalMove::CastleKingside {
                castle_mask: match self.active_player {
                    Color::White => self.castling_rights & 0b1100,
                    Color::Black => self.castling_rights & 0b0011,
                },
            });
        }
        if can_castle_queenside(self) {
            legal_moves.push(LegalMove::CastleQueenside {
                castle_mask: match self.active_player {
                    Color::White => self.castling_rights & 0b1100,
                    Color::Black => self.castling_rights & 0b0011,
                },
            });
        }
        legal_moves
    }

    fn has_legal_moves(&self) -> bool {
        let (forward, home_rank, opp_home_rank) = match self.active_player {
            Color::White => (1, Rank::_2, Rank::_7),
            Color::Black => (-1, Rank::_7, Rank::_2),
        };
        for (piece, src) in PlayerPieceIter::new(self, self.active_player) {
            let mk_ray = move |dir: (i8, i8)| RayIter::new(src, dir);
            if match piece {
                PieceType::Queen => DirIter::all()
                    .flat_map(mk_ray)
                    .any(|dest| is_move_legal(self, &Move::Normal { src, dest })),
                PieceType::Rook => DirIter::rook()
                    .flat_map(mk_ray)
                    .any(|dest| is_move_legal(self, &Move::Normal { src, dest })),
                PieceType::Bishop => DirIter::bishop()
                    .flat_map(mk_ray)
                    .any(|dest| is_move_legal(self, &Move::Normal { src, dest })),
                PieceType::Knight => KnightHopIter::new(src)
                    .any(|dest| is_move_legal(self, &Move::Normal { src, dest })),
                PieceType::King => DirIter::all()
                    .flat_map(|dir| pos_plus(src, dir))
                    .any(|dest| is_move_legal(self, &Move::Normal { src, dest })),
                PieceType::Pawn => {
                    if src.1 == opp_home_rank {
                        // promotion
                        [(0, forward), (-1, forward), (1, forward)]
                            .iter()
                            .filter_map(move |step| pos_plus(src, *step))
                            .any(|dest| {
                                is_move_legal(
                                    self,
                                    &Move::Promotion {
                                        src,
                                        dest,
                                        promotion: PromotionPieceType::Queen,
                                    },
                                )
                            })
                    } else if home_rank == src.1 {
                        //include double pawn push
                        [(0, forward), (-1, forward), (1, forward), (0, 2 * forward)]
                            .iter()
                            .filter_map(move |step| pos_plus(src, *step))
                            .any(|dest| is_move_legal(self, &Move::Normal { src, dest }))
                    } else {
                        //exclude double pawn push
                        [(0, forward), (-1, forward), (1, forward)]
                            .iter()
                            .filter_map(move |step| pos_plus(src, *step))
                            .any(|dest| is_move_legal(self, &Move::Normal { src, dest }))
                    }
                }
            } {
                return true;
            }
        }
        if is_move_legal(self, &Move::CastleKingside {}) {
            return true;
        }
        if is_move_legal(self, &Move::CastleQueenside {}) {
            return true;
        }
        false
    }

    pub fn get_gamestate(&self) -> GameState {
        if !self.has_legal_moves() {
            if is_king_in_check(self) {
                GameState::Mated(self.active_player)
            } else {
                GameState::Stalemate
            }
        } else {
            GameState::InProgress
        }
    }

    fn move_piece(&mut self, src: Square, dest: Square) {
        let piece = self.get_piece_at(src).unwrap();
        self.set_piece_at(dest, piece);
        self.clear_square(src);
    }

    pub fn make_move(&mut self, move_: &LegalMove) {
        let active_player = self.active_player;
        self.en_passant_target = None;
        match move_ {
            LegalMove::Normal {
                src,
                dest,
                castle_mask,
                ..
            } => {
                self.move_piece(*src, *dest);
                self.castling_rights ^= castle_mask;
            }
            LegalMove::Promotion {
                src,
                dest,
                castle_mask,
                promotion,
                ..
            } => {
                self.set_piece_at(*dest, Piece(*promotion, active_player));
                self.clear_square(*src);
                self.castling_rights ^= castle_mask;
            }
            LegalMove::CastleKingside { castle_mask } => {
                self.move_piece(
                    active_player.king_home_square(),
                    Square(File::G, active_player.home_rank()),
                );
                self.move_piece(
                    Square(File::H, active_player.home_rank()),
                    Square(File::F, active_player.home_rank()),
                );
                self.castling_rights ^= castle_mask;
            }
            LegalMove::CastleQueenside { castle_mask } => {
                self.move_piece(
                    active_player.king_home_square(),
                    Square(File::C, active_player.home_rank()),
                );
                self.move_piece(
                    Square(File::A, active_player.home_rank()),
                    Square(File::D, active_player.home_rank()),
                );
                self.castling_rights ^= castle_mask;
            }
            LegalMove::DoublePawnPush { file, .. } => {
                let (src_rank, target_rank, dst_rank) = match active_player {
                    Color::White => (Rank::_2, Rank::_3, Rank::_4),
                    Color::Black => (Rank::_7, Rank::_6, Rank::_5),
                };
                self.set_piece_at(
                    Square(*file, dst_rank),
                    Piece(PieceType::Pawn, self.active_player),
                );
                self.clear_square(Square(*file, src_rank));
                self.en_passant_target = Some(Square(*file, target_rank));
            }
            LegalMove::EnPassantCapture { src, dest } => {
                self.move_piece(*src, *dest);
                self.clear_square(Square(dest.0, active_player.opponent().double_push_rank()));
            }
        }
        self.active_player = active_player.opponent();
    }

    // TODO: Unmake move currently DOES NOT restore the en passant target
    // This means that in general, the board cannot be relied to give correct legal moves after unmake_move
    // Or rather, the board is reliable again after a forward-move (make_move) has been made
    // However, moves have been encoded in a way that allows previously (correctly) generated en passant captures to be played
    pub fn unmake_move(&mut self, move_: &LegalMove) {
        self.active_player = self.active_player.opponent();
        let active_player = self.active_player;
        self.en_passant_target = None;
        match move_ {
            LegalMove::Normal {
                src,
                dest,
                castle_mask,
                captured_piece,
            } => {
                let my_piece = self.get_piece_at(*dest);
                self.set_square(
                    *dest,
                    captured_piece.map(|p| Piece(p, active_player.opponent())),
                );
                self.set_square(*src, my_piece);
                self.castling_rights ^= castle_mask;
            }
            LegalMove::Promotion {
                src,
                dest,
                castle_mask,
                captured_piece,
                ..
            } => {
                self.set_piece_at(*src, Piece(PieceType::Pawn, active_player));
                self.set_square(
                    *dest,
                    captured_piece.map(|p| Piece(p, active_player.opponent())),
                );
                self.castling_rights ^= castle_mask;
            }
            LegalMove::CastleKingside { castle_mask } => {
                self.move_piece(
                    Square(File::G, active_player.home_rank()),
                    active_player.king_home_square(),
                );
                self.move_piece(
                    Square(File::F, active_player.home_rank()),
                    Square(File::H, active_player.home_rank()),
                );
                self.castling_rights ^= castle_mask;
            }
            LegalMove::CastleQueenside { castle_mask } => {
                self.move_piece(
                    Square(File::C, active_player.home_rank()),
                    active_player.king_home_square(),
                );
                self.move_piece(
                    Square(File::D, active_player.home_rank()),
                    Square(File::A, active_player.home_rank()),
                );
                self.castling_rights ^= castle_mask;
            }
            LegalMove::DoublePawnPush { file } => {
                self.set_piece_at(
                    Square(*file, active_player.pawn_start_rank()),
                    Piece(PieceType::Pawn, self.active_player),
                );
                self.clear_square(Square(*file, active_player.double_push_rank()));
            }
            LegalMove::EnPassantCapture { src, dest } => {
                self.set_piece_at(*src, self.get_piece_at(*dest).unwrap());
                self.clear_square(*dest);
                // put back the captured pawn
                self.set_piece_at(
                    Square(dest.0, active_player.opponent().double_push_rank()),
                    Piece(PieceType::Pawn, active_player.opponent()),
                );
            }
        }
    }
}
