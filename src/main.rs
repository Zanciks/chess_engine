use shakmaty::{Chess, Position};
mod evaluation;

fn main() {
    let pos = Chess::default();
    println!("{}", evaluation::evaluate(pos.board()));
}