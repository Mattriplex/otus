use std::fmt::{self, Display};

use crate::chess::models::Piece;

use self::{
    models::{Color, File, GameState, Move, PieceType, Pos, PromotionPieceType, Rank},
    player::ChessPlayer,
};

pub mod move_checking;

pub mod models;
pub mod player;

#[cfg(test)]
mod tests;

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PieceType::Pawn => write!(f, "Pawn"),
            PieceType::Knight => write!(f, "Knight"),
            PieceType::Bishop => write!(f, "Bishop"),
            PieceType::Rook => write!(f, "Rook"),
            PieceType::Queen => write!(f, "Queen"),
            PieceType::King => write!(f, "King"),
        }
    }
}

pub trait Opponent {
    fn opponent(&self) -> Self;
}

impl Opponent for Color {
    fn opponent(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Color::White => "White",
            Color::Black => "Black",
        };
        write!(f, "{}", s)
    }
}

impl Rank {
    fn from_i8(i: i8) -> Option<Rank> {
        match i {
            0 => Some(Rank::_1),
            1 => Some(Rank::_2),
            2 => Some(Rank::_3),
            3 => Some(Rank::_4),
            4 => Some(Rank::_5),
            5 => Some(Rank::_6),
            6 => Some(Rank::_7),
            7 => Some(Rank::_8),
            _ => None,
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as u8 + 1)
    }
}

impl File {
    fn from_i8(i: i8) -> Option<File> {
        match i {
            0 => Some(File::A),
            1 => Some(File::B),
            2 => Some(File::C),
            3 => Some(File::D),
            4 => Some(File::E),
            5 => Some(File::F),
            6 => Some(File::G),
            7 => Some(File::H),
            _ => None,
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (*self as u8 + 'a' as u8) as char)
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Piece(PieceType::Pawn, Color::White) => '♙',
            Piece(PieceType::Knight, Color::White) => '♘',
            Piece(PieceType::Bishop, Color::White) => '♗',
            Piece(PieceType::Rook, Color::White) => '♖',
            Piece(PieceType::Queen, Color::White) => '♕',
            Piece(PieceType::King, Color::White) => '♔',
            Piece(PieceType::Pawn, Color::Black) => '♟',
            Piece(PieceType::Knight, Color::Black) => '♞',
            Piece(PieceType::Bishop, Color::Black) => '♝',
            Piece(PieceType::Rook, Color::Black) => '♜',
            Piece(PieceType::Queen, Color::Black) => '♛',
            Piece(PieceType::King, Color::Black) => '♚',
        };
        write!(f, "{}", symbol)
    }
}

impl Pos {
    fn from_string(s: &str) -> Result<Pos, String> {
        if s.len() != 2 {
            return Err("Expected 2 characters".to_string());
        }
        let file = match s.chars().nth(0).unwrap().to_ascii_lowercase() {
            'a' => Ok(File::A),
            'b' => Ok(File::B),
            'c' => Ok(File::C),
            'd' => Ok(File::D),
            'e' => Ok(File::E),
            'f' => Ok(File::F),
            'g' => Ok(File::G),
            'h' => Ok(File::H),
            _ => Err("Invalid file".to_string()),
        }?;
        let rank = match s.chars().nth(1).unwrap() {
            '1' => Ok(Rank::_1),
            '2' => Ok(Rank::_2),
            '3' => Ok(Rank::_3),
            '4' => Ok(Rank::_4),
            '5' => Ok(Rank::_5),
            '6' => Ok(Rank::_6),
            '7' => Ok(Rank::_7),
            '8' => Ok(Rank::_8),
            _ => Err("Invalid rank".to_string()),
        }?;
        Ok(Pos(file, rank))
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
    active_player: Color,
    castling_rights: u8, // KQkq
    en_passant_target: Option<Pos>,
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

    fn get_piece_at(&self, pos: Pos) -> Option<Piece> {
        self.get_piece(pos.0, pos.1)
    }

    fn set_piece(&mut self, file: File, rank: Rank, piece: Piece) {
        self.squares[rank as usize][file as usize] = Some(piece);
    }

    fn set_piece_at(&mut self, pos: Pos, piece: Piece) {
        self.squares[pos.1 as usize][pos.0 as usize] = Some(piece);
    }

    fn clear_square(&mut self, pos: Pos) {
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
            s => Some(Pos::from_string(s)?),
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
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rank in (0..8).rev() {
            self.fmt_rank(f, rank)?;
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::Normal {
                src: from,
                dest: to,
            } => write!(f, "{}-{}", from, to),
            Move::CastleKingside => write!(f, "0-0"),
            Move::CastleQueenside => write!(f, "0-0-0"),
            Move::Promotion {
                src: from,
                dest: to,
                promotion,
            } => {
                write!(
                    f,
                    "{}-{}={}",
                    from,
                    to,
                    match promotion {
                        PromotionPieceType::Knight => "N",
                        PromotionPieceType::Bishop => "B",
                        PromotionPieceType::Rook => "R",
                        PromotionPieceType::Queen => "Q",
                    }
                )
            }
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
                match move_checking::get_gamestate(&board) {
                    GameState::InProgress => (),
                    gs => return gs,
                }
            }
            // Illegal move, game is forfeit
            Err(_) => return GameState::Mated(board.active_player),
        }
    }
}
