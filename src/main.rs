use shakmaty::{Chess, Position};
mod evaluation;
mod piece_square_tables;

fn main() {
    let pos = Chess::default();
    println!("{}", evaluation::evaluate(pos.board()));
}