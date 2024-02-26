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

#[derive(Debug, Copy, Clone, PartialEq)]
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
pub struct Square(pub File, pub Rank);

#[derive(Debug, Clone, PartialEq)]
pub enum Move {
    Normal {
        src: Square,
        dest: Square,
    },
    CastleKingside,
    CastleQueenside,
    Promotion {
        src: Square,
        dest: Square,
        promotion: PromotionPieceType,
    },
}

/* This move is legal in the context of a specific board. Care must be taken to not apply a LegalMove to the wrong board.
This way, we can skip the legality checks when applying the move.
The LegalMove is designed in a way to allow all moves to be reversed, and to require minimal computation to apply.
TODO: model captures for 50 move rule

Users should NEVER construct a Legalmove directly. Instead, use the Board's get_legal_moves function to get a list of legal moves.
*/
#[derive(Debug, Clone, PartialEq)]
pub enum LegalMove {
    Normal {
        src: Square,
        dest: Square,
        castle_mask: u8, // relevant for: Non-castling king moves, Rook moves, rook captures
                         // castle mask will be XOR-ed against the board's castling rights
    },
    DoublePawnPush {
        file: File,
    },
    CastleKingside {
        castle_mask: u8
    },
    CastleQueenside {
        castle_mask: u8
    },
    Promotion {
        src: Square,
        dest: Square,
        castle_mask: u8, // in case the pawn captures a rook
        promotion: PromotionPieceType,
    },
    EnPassantCapture {
        src: Square, //dest is given by boards en passant square
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    InProgress,
    Mated(Color),
    Stalemate,
}
