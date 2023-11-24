use chess;
use chess::Board;

mod uci;
mod calculate;

fn main() {
    let mut board = Board::default();
    loop {
        uci::commands(&mut board);
    }
}
