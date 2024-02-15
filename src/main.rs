use std::str::FromStr;

mod evaluation;
mod calculation;
mod tests;

fn main() {
    let board = chess::Board::from_str("7r/8/3k4/8/5K2/8/1r6/8 b - - 12 7").unwrap();
    let (eval, mv) = calculation::find_best_move(board, 5);
    println!("{} | {}", eval, mv);
    evaluation::evaluate(board);
}