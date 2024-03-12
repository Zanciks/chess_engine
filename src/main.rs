use std::str::FromStr;

mod evaluation;
mod calculation;
mod tests;

fn main() {
    let board = chess::Board::from_str("4rBk1/2N2pPp/1p6/1Knb1Nb1/r5P1/8/1P6/6RR b - - 9 37").unwrap();
    let (eval, mv) = calculation::find_best_move(board, 7);
    println!("{} | {}", eval, mv);
}