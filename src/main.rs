use chess::{self, Board};

mod uci;
mod calculate;

fn main() {
    run_engine();
}

fn run_engine() {
    let mut board = Board::default();
    loop {
        uci::commands(&mut board);
    }
}