use std::fmt::{self, Display};

use crate::chess::move_checking::is_move_legal;

mod move_checking;
#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, PartialEq)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

enum PromotionPieceType {
    Knight,
    Bishop,
    Rook,
    Queen,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
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

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
enum Rank {
    _1 = 0,
    _2 = 1,
    _3 = 2,
    _4 = 3,
    _5 = 4,
    _6 = 5,
    _7 = 6,
    _8 = 7,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Piece(PieceType, Color);

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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position(File, Rank);

impl Position {
    fn from_string(s: &str) -> Result<Position, String> {
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
        Ok(Position(file, rank))
    }
}

pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
    active_player: Color,
    castling_rights: u8, // KQkq
    en_passant_target: Option<Position>,
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

    fn set_piece(&mut self, file: File, rank: Rank, piece: Piece) {
        self.squares[rank as usize][file as usize] = Some(piece);
    }

    fn clear_square(&mut self, file: File, rank: Rank) {
        self.squares[rank as usize][file as usize] = None;
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
            s => Some(Position::from_string(s)?),
        };
        Ok(Board {
            squares,
            active_player,
            castling_rights,
            en_passant_target,
        })
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

pub enum Move {
    Normal {
        from: Position,
        to: Position,
    },
    CastleKingside,
    CastleQueenside,
    Promotion {
        from: Position,
        to: Position,
        promotion: PromotionPieceType,
    },
}

pub enum GameState {
    InProgress,
    Mated(Color),
    Stalemate,
}

pub trait ChessPlayer {
    fn make_move(&self, board: &Board) -> Move;
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

pub struct HumanPlayer;

impl HumanPlayer {
    fn try_get_move_input(&self, board: &Board) -> Result<Move, String> {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "0-0" => Ok(Move::CastleKingside),
            "0-0-0" => Ok(Move::CastleQueenside),
            s => {
                let parts: Vec<&str> = s.split_whitespace().collect();
                if parts.len() != 2 {
                    return Err("Error parsing move".to_string());
                }
                let from = Position::from_string(parts[0]).unwrap();
                let to = Position::from_string(parts[1]).unwrap();
                let move_ = Move::Normal { from, to };
                if is_move_legal(board, &move_) {
                    Ok(move_)
                } else {
                    Err("Illegal move".to_string())
                }
            }
        }
    }
}

impl ChessPlayer for HumanPlayer {
    // TODO promotion move
    fn make_move(&self, board: &Board) -> Move {
        println!("{}", board);
        println!("You are {}. Enter your move: ", board.active_player);
        let mut input = String::new();
        loop {
            match self.try_get_move_input(board) {
                Ok(m) => return m,
                Err(e) => println!("{}, try again!", e),
            }
        }
    }
}