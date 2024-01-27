mod evaluation;
mod calculation;

use std::str::FromStr;

use chess::*;

fn main() {
    let board = Board::from_str("3r1knr/pP3ppp/5q2/2b5/Q3p3/8/PPP2PPP/RNB2RK1 b - - 2 15").unwrap();
    let (eval, mv) = calculation::find_best_move(board, 3);
    println!("{}, {}", eval, mv);
}
