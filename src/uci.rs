use chess::Board;
use std::{io, time::Instant, str::FromStr};
use crate::calculate::find_best_move;

pub fn commands(board: &mut Board) {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    uci(board, input_string.trim());
}

// We are only making UCI its own function so that commands() can be called in a loop
// and so that uci() can also be used to pass in hard-coded commands, if its ever needed.
pub fn uci(board: &mut Board, command: &str) {
    let args: Vec<&str> = command.split_whitespace().collect();
    match args.first() {
        Some(&"uci") => uci_ok(),
        Some(&"isready") => println!("readyok"),
        Some(&"go") => go(board, args.iter().skip(1).cloned().collect()),
        Some(&"position") => *board = position(args.iter().skip(1).cloned().collect()),
        Some(other) => println!("Unknown command: {}", other),
        None => (),
    }
}

fn uci_ok() {
    println!("id name Rust Chess Engine");
    println!("id author Zanciks");
    println!("uciok");
}

fn go(board: &Board, commands: Vec<&str>) {
    let mut max_depth: u8 = 2; // default max_depth, just so it always calculates at least 1 move.
    if commands.iter().next() == Some(&"depth") {
        // this will panic if we can't find a number. However, for any UCI program,
        // this should always be a number, assuming correct UCI protocol.g
        max_depth = commands.iter().skip(1).next().unwrap().parse().unwrap();
    }
    let time = Instant::now();
    for depth in 1..max_depth {
        let (eval, mv) = find_best_move(board, depth);
        println!("info depth {} score {} nodes {} time {} pv {}", depth, eval, 1000, time.elapsed().as_millis(), mv);
    }
    let (eval, mv) = find_best_move(board, max_depth);
        println!("info depth {} score {} nodes {} time {} pv {}", max_depth, eval, 1000, time.elapsed().as_millis(), mv);
    println!("bestmove {}", mv);
}

fn position(strings: Vec<&str>) -> Board {
    let fen_string: String = strings.iter().take(6).cloned().collect::<Vec<&str>>().join(" ");
    return Board::from_str(fen_string.as_str()).unwrap();
}