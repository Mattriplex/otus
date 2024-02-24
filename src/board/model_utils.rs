use std::fmt::{self, Display};

use crate::board::models::{Color, Piece, PromotionPieceType};

use super::{
    models::{File, LegalMove, Move, PieceType, Rank, Square},
    Board,
};

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
    pub fn from_i8(i: i8) -> Option<Rank> {
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
    // TODO possible optimisation: work at usage sites with unreachable!() instead of Option
    pub fn from_i8(i: i8) -> Option<File> {
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

impl Square {
    pub fn from_string(s: &str) -> Result<Square, String> {
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
        Ok(Square(file, rank))
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::Normal {
                src: from,
                dest: to,
            } => write!(f, "{}{}", from, to),
            Move::CastleKingside => write!(f, "0-0"), //TODO make uci compliant
            Move::CastleQueenside => write!(f, "0-0-0"),
            Move::Promotion {
                src: from,
                dest: to,
                promotion,
            } => {
                write!(
                    f,
                    "{}{}{}",
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

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rank in (0..8).rev() {
            self.fmt_rank(f, rank)?;
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Move {
    //no spaces,
    pub fn from_uci_string(board: &Board, move_str: &str) -> Result<Move, String> {
        if move_str.len() < 4 {
            return Err("Error parsing move".to_string());
        }
        let from = Square::from_string(&move_str[0..2])?;
        let to = Square::from_string(&move_str[2..4])?;
        if move_str == "e1g1"
            && Some(Piece(PieceType::King, Color::White)) == board.get_piece_at(from)
        {
            return Ok(Move::CastleKingside);
        }
        if move_str == "e1c1"
            && Some(Piece(PieceType::King, Color::White)) == board.get_piece_at(from)
        {
            return Ok(Move::CastleQueenside);
        }
        if move_str == "e8g8"
            && Some(Piece(PieceType::King, Color::Black)) == board.get_piece_at(from)
        {
            return Ok(Move::CastleKingside);
        }
        if move_str == "e8c8"
            && Some(Piece(PieceType::King, Color::Black)) == board.get_piece_at(from)
        {
            return Ok(Move::CastleQueenside);
        }
        if move_str.len() == 5 {
            let promotion = match &move_str[4..5] {
                "n" => PromotionPieceType::Knight,
                "b" => PromotionPieceType::Bishop,
                "r" => PromotionPieceType::Rook,
                "q" => PromotionPieceType::Queen,
                _ => return Err("Invalid promotion piece".to_string()),
            };
            Ok(Move::Promotion {
                src: from,
                dest: to,
                promotion,
            })
        } else {
            Ok(Move::Normal {
                src: from,
                dest: to,
            })
        }
    }
}

impl LegalMove {
    pub fn to_move(&self, board: &Board) -> Move {
        match self {
            LegalMove::Normal { src, dest, .. } => Move::Normal {
                src: *src,
                dest: *dest,
            },
            LegalMove::CastleKingside => Move::CastleKingside,
            LegalMove::CastleQueenside => Move::CastleQueenside,
            LegalMove::Promotion {
                src,
                dest,
                promotion,
                ..
            } => Move::Promotion {
                src: *src,
                dest: *dest,
                promotion: *promotion,
            },
            LegalMove::EnPassantCapture { src } => Move::Normal {
                src: *src,
                dest: board.en_passant_target.expect("En passant target empty"),
            },
            LegalMove::DoublePawnPush { file } => match board.active_player {
                Color::White => Move::Normal {
                    src: Square(*file, Rank::_2),
                    dest: Square(*file, Rank::_4),
                },
                Color::Black => Move::Normal {
                    src: Square(*file, Rank::_7),
                    dest: Square(*file, Rank::_5),
                },
            },
        }
    }
}
