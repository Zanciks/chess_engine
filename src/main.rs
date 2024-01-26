mod evaluation;
mod calculation;

use std::str::FromStr;

use calculation::calculate;
use chess::*;

fn main() {
    let board = Board::from_str("rnbqkbnr/pppp1ppp/4p3/8/6P1/5P2/PPPPP2P/RNBQKBNR b KQkq - 0 1").unwrap();
    let (eval, mv) = calculate(board);
    println!("{}, {}", eval, mv); 
}
