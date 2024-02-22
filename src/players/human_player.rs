use crate::board::{
    models::{Move, PromotionPieceType, Square},
    move_checking::{is_move_legal, is_promotion_move},
    Board,
};

use super::{ChessPlayer, HumanPlayer};

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
                let from = Square::from_string(parts[0]).unwrap();
                let to = Square::from_string(parts[1]).unwrap();
                let move_;
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
                    move_ = Move::Promotion {
                        src: from,
                        dest: to,
                        promotion,
                    };
                } else {
                    move_ = Move::Normal {
                        src: from,
                        dest: to,
                    };
                }
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
    fn make_move(&self, board: &Board) -> Move {
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
