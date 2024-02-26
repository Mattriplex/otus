use crate::board::{
    models::{LegalMove, Move, PromotionPieceType, Square},
    move_checking::{get_legal_move_from_move, is_promotion_move},
    Board,
};

use super::{ChessPlayer, HumanPlayer};

impl HumanPlayer {
    fn try_get_move_input(&self, board: &Board) -> Result<LegalMove, String> {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let candidate = match input.trim() {
            "0-0" => Move::CastleKingside,
            "0-0-0" => Move::CastleQueenside,
            s => {
                let parts: Vec<&str> = s.split_whitespace().collect();
                if parts.len() != 2 {
                    return Err("Error parsing move".to_string());
                }
                let from = Square::from_string(parts[0]).unwrap();
                let to = Square::from_string(parts[1]).unwrap();
                if is_promotion_move(board, from, to) {
                    println!("Enter promotion piece: ");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let promotion = match input.trim() {
                        "n" => PromotionPieceType::Knight,
                        "b" => PromotionPieceType::Bishop,
                        "r" => PromotionPieceType::Rook,
                        "q" => PromotionPieceType::Queen,
                        _ => return Err("Invalid promotion piece".to_string()),
                    };
                    Move::Promotion {
                        src: from,
                        dest: to,
                        promotion,
                    }
                } else {
                    Move::Normal {
                        src: from,
                        dest: to,
                    }
                }
            }
        };
        if let Some(legal_move) = get_legal_move_from_move(board, &candidate) {
            Ok(legal_move)
        } else {
            Err("Illegal move".to_string())
        }
    }
}

impl ChessPlayer for HumanPlayer {
    fn propose_move(&self, board: &Board) -> LegalMove {
        println!("{}", board);
        println!("You are {}. Enter your move: ", board.active_player);
        loop {
            match self.try_get_move_input(board) {
                Ok(m) => return m,
                Err(e) => println!("{}, try again!", e),
            }
        }
    }
}
