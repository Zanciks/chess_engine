use std::str::FromStr;

mod evaluation;
mod calculation;
mod tests;

fn main() {
    let board = chess::Board::from_str("7k/8/8/8/8/8/8/R6K w - - 0 1").unwrap();
    let (eval, mv) = calculation::find_best_move(board, 5);
    println!("{} | {}", eval, mv);
    evaluation::evaluate(board);
}