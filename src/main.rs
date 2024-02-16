mod chess;

fn main() {
    println!("Hello, world!");
    let board = chess::Board::default();
    println!("{}", board);
}
