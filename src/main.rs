mod evaluation;
mod calculation;

use std::str::FromStr;

use chess::*;

fn main() {
    let board = Board::from_str("r4rk1/pp1n2p1/2pb2qp/3p1p2/3P2bN/1NPBB3/PPQ2PK1/R6R b - - 5 17").unwrap();
    let (eval, mv) = calculation::find_best_move(board, 3);
    println!("{}, {}", eval, mv); 
}
