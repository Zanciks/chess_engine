mod evaluation;
mod calculation;
mod tests;

use std::str::FromStr;

use chess::*;

fn main() {
    let board = Board::from_str("r2q1r1k/1p2Np1p/p1pp2n1/5bQ1/7R/5P2/PPP3PP/R5K1 w - - 0 23").unwrap();
    let (eval, mv) = calculation::find_best_move(board, 3);
    println!("{}, {}", eval, mv);
}
