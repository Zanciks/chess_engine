use std::str::FromStr;

mod evaluation;
mod calculation;
mod tests;

fn main() {
    let board = chess::Board::from_str("3q3k/8/8/8/8/8/8/3Q3K b - - 0 1").unwrap();
    let (eval, mv) = calculation::find_best_move(board, 1);
    println!("{} | {}", eval, mv);
}