use chess::Board;
use std::{io, time::Instant, str::FromStr};
use crate::calculate::find_best_move;

pub fn commands(board: &mut Board) {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");

    match input_string.trim().split_whitespace().next() {
        Some("uci") => uci(),
        Some("isready") => println!("readyok"),
        Some("go") => go(board),
        Some("position") => *board = position(input_string.trim().split_whitespace().skip(1).collect()),
        Some(other) => println!("Unknown command: {}", other),
        None => (),
    }
}

fn uci() {
    println!("id name Rust Chess Engine");
    println!("id author Zanciks");
    println!("uciok");
}

fn go(board: &Board) {
    let max_depth = 3;
    let time = Instant::now();
    for depth in 1..max_depth {
        let (eval, mv) = find_best_move(board, depth);
        println!("info depth {} score {} nodes {} nps {} time {} pv {}", depth, eval, 1000, 1000, time.elapsed().as_millis(), mv);
    }
    let (eval, mv) = find_best_move(board, max_depth);
    println!("info depth {} score {} nodes {} nps {} time {} pv {}", max_depth, eval, 1000, 1000, time.elapsed().as_millis(), mv);
    println!("bestmove {}", mv);
}

fn position(strings: Vec<&str>) -> Board {
    let fen_string: String = strings.iter().take(6).cloned().collect::<Vec<&str>>().join(" ");
    return Board::from_str(fen_string.as_str()).unwrap();
}