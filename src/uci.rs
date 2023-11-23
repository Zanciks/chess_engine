use std::io;

pub fn commands() {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");

    match input_string.as_str() {
        "uci" => uci(),
        "isready" => println!("readyok"),
        "go" => go(),
        other => println!("Unknown command: {}", other),
    }
}

fn uci() {
    println!("id name Rust Chess Engine");
    println!("id author Zanciks");
    println!("uciok");
}

fn go() {
    
}