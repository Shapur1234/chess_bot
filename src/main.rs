use crate::chess_logic::Board;

mod chess_logic;

fn main() {
    let board = Board::new();
    println!("{board:}");
}
