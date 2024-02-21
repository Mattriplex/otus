#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub enum PromotionPieceType {
    Knight,
    Bishop,
    Rook,
    Queen,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Rank {
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
pub enum File {
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
pub struct Piece(pub PieceType, pub Color);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pos(pub File, pub Rank);

pub enum Move {
    Normal {
        src: Pos,
        dest: Pos,
    },
    CastleKingside,
    CastleQueenside,
    Promotion {
        src: Pos,
        dest: Pos,
        promotion: PromotionPieceType,
    },
}

pub enum GameState {
    InProgress,
    Mated(Color),
    Stalemate,
}
