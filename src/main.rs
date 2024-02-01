mod evaluation;
mod calculation;

use std::str::FromStr;

use chess::*;

fn main() {
    let board = Board::from_str("4R3/pp1r2kp/1q3pp1/8/2Q5/7P/P4PP1/5K2 w - - 0 29").unwrap();
    let (eval, mv) = calculation::find_best_move(board, 3);
    println!("{}, {}", eval, mv);
}
