use chess;
use::std::str::FromStr;
mod uci;
mod calculate;
mod tests;

fn main() {
    let board = chess::Board::from_str("rnbqkbnr/ppppp2p/5p2/6p1/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 3").unwrap();
    calculate::find_best_move(&board, 1);
}
