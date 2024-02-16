fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Color {
    White,
    Black,
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

struct Position (File, Rank);

impl Position {
    fn from_string(s: &str) -> Result<Position, String> {
        if s.len() != 2 {
            return Err("Expected 2 characters".to_string());
        }
        let file = match s.chars().nth(0).unwrap() {
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

struct Board {
    squares: [[Option<Piece>; 8]; 8],
    active_player: Color,
    castling_rights: u8, // KQkq
    en_passant_target: Option<Position>,
}

impl Board {

    fn default() -> Board {
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
        let mut file: usize= 0;
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
        self.castling_rights & match color {
            Color::White => 0b1000,
            Color::Black => 0b0010,
        } != 0
    }

    fn can_castle_queenside(&self, color: Color) -> bool {
        self.castling_rights & match color {
            Color::White => 0b0100,
            Color::Black => 0b0001,
        } != 0
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_fen_default_board() {
        use Color::{*};
        use PieceType::{*};
        use Rank::{*};
        use File::{*};

        let board: Board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        assert_eq!(board.get_piece(A, _1), Some(Piece(Rook, White)));
        assert_eq!(board.get_piece(A, _2), Some(Piece(Pawn, White)));
        assert!(board.get_piece(A, _3).is_none());
        assert!(board.get_piece(A, _4).is_none());
        assert!(board.get_piece(A, _5).is_none());
        assert!(board.get_piece(A, _6).is_none());
        assert_eq!(board.get_piece(A, _7), Some(Piece(Pawn, Black)));
        assert_eq!(board.get_piece(A, _8), Some(Piece(Rook, Black)));

        assert_eq!(board.get_piece(B, _1), Some(Piece(Knight, White)));
        assert_eq!(board.get_piece(B, _2), Some(Piece(Pawn, White)));
        assert!(board.get_piece(B, _3).is_none());
        assert!(board.get_piece(B, _4).is_none());
        assert!(board.get_piece(B, _5).is_none());
        assert!(board.get_piece(B, _6).is_none());
        assert_eq!(board.get_piece(B, _7), Some(Piece(Pawn, Black)));
        assert_eq!(board.get_piece(B, _8), Some(Piece(Knight, Black)));

        assert_eq!(board.get_piece(C, _1), Some(Piece(Bishop, White)));
        assert_eq!(board.get_piece(C, _2), Some(Piece(Pawn, White)));
        assert!(board.get_piece(C, _3).is_none());
        assert!(board.get_piece(C, _4).is_none());
        assert!(board.get_piece(C, _5).is_none());
        assert!(board.get_piece(C, _6).is_none());
        assert_eq!(board.get_piece(C, _7), Some(Piece(Pawn, Black)));
        assert_eq!(board.get_piece(C, _8), Some(Piece(Bishop, Black)));

        assert_eq!(board.get_piece(D, _1), Some(Piece(Queen, White)));
        assert_eq!(board.get_piece(D, _2), Some(Piece(Pawn, White)));
        assert!(board.get_piece(D, _3).is_none());
        assert!(board.get_piece(D, _4).is_none());
        assert!(board.get_piece(D, _5).is_none());
        assert!(board.get_piece(D, _6).is_none());
        assert_eq!(board.get_piece(D, _7), Some(Piece(Pawn, Black)));
        assert_eq!(board.get_piece(D, _8), Some(Piece(Queen, Black)));

        assert_eq!(board.get_piece(E, _1), Some(Piece(King, White)));
        assert_eq!(board.get_piece(E, _2), Some(Piece(Pawn, White)));
        assert!(board.get_piece(E, _3).is_none());
        assert!(board.get_piece(E, _4).is_none());
        assert!(board.get_piece(E, _5).is_none());
        assert!(board.get_piece(E, _6).is_none());
        assert_eq!(board.get_piece(E, _7), Some(Piece(Pawn, Black)));
        assert_eq!(board.get_piece(E, _8), Some(Piece(King, Black)));

        assert_eq!(board.get_piece(F, _1), Some(Piece(Bishop, White)));
        assert_eq!(board.get_piece(F, _2), Some(Piece(Pawn, White)));
        assert!(board.get_piece(F, _3).is_none());
        assert!(board.get_piece(F, _4).is_none());
        assert!(board.get_piece(F, _5).is_none());
        assert!(board.get_piece(F, _6).is_none());
        assert_eq!(board.get_piece(F, _7), Some(Piece(Pawn, Black)));
        assert_eq!(board.get_piece(F, _8), Some(Piece(Bishop, Black)));

        assert_eq!(board.get_piece(G, _1), Some(Piece(Knight, White)));
        assert_eq!(board.get_piece(G, _2), Some(Piece(Pawn, White)));
        assert!(board.get_piece(G, _3).is_none());
        assert!(board.get_piece(G, _4).is_none());
        assert!(board.get_piece(G, _5).is_none());
        assert!(board.get_piece(G, _6).is_none());
        assert_eq!(board.get_piece(G, _7), Some(Piece(Pawn, Black)));
        assert_eq!(board.get_piece(G, _8), Some(Piece(Knight, Black)));

        assert_eq!(board.get_piece(H, _1), Some(Piece(Rook, White)));
        assert_eq!(board.get_piece(H, _2), Some(Piece(Pawn, White)));
        assert!(board.get_piece(H, _3).is_none());
        assert!(board.get_piece(H, _4).is_none());
        assert!(board.get_piece(H, _5).is_none());
        assert!(board.get_piece(H, _6).is_none());
        assert_eq!(board.get_piece(H, _7), Some(Piece(Pawn, Black)));
        assert_eq!(board.get_piece(H, _8), Some(Piece(Rook, Black)));

        assert_eq!(board.active_player, White);

        assert!(board.can_castle_kingside(White));
        assert!(board.can_castle_queenside(White));
        assert!(board.can_castle_kingside(Black));
        assert!(board.can_castle_queenside(Black));
    }
}
