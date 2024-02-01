mod evaluation;
mod calculation;
mod tests;

use std::str::FromStr;

use chess::*;

fn main() {
    let board = Board::from_str("3r4/8/2k5/8/2K5/8/4r3/8 b - - 6 4").unwrap();
    let (eval, mv) = calculation::find_best_move(board, 5);
    println!("{}, {}", eval, mv);
}
