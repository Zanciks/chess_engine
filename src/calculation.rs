#![allow(dead_code, unused_variables)]

use chess::{Board, BoardStatus, ChessMove, Color, MoveGen};
use crate::evaluation::evaluate;

pub fn find_best_move(board: Board, depth: u8) -> (i32, ChessMove) {
    let maximizing = if board.side_to_move() == Color::White {true} else {false};
    let (best_eval, best_move) = minimax(board, depth, maximizing);

    return (best_eval, best_move);
}

pub fn minimax(board: Board, depth: u8, maximizing: bool) -> (i32, ChessMove) {
    if board.status() == BoardStatus::Checkmate {return (-i32::MAX, ChessMove::default())}
    if depth == 0 {return (evaluate(board), ChessMove::default())}

    let mut best_move = ChessMove::default();
    let mut best_eval: i32;
    if maximizing {best_eval = -i32::MAX} else {best_eval = i32::MAX}

    let color_factor = if board.side_to_move() == chess::Color::White { 1 } else { -1 };

    for mv in MoveGen::new_legal(&board) {
        let new_board = board.make_move_new(mv);
        let eval = minimax(new_board, depth - 1, !maximizing).0;
        if eval * color_factor > best_eval * color_factor {
            best_eval = eval;
            best_move = mv;
        }
    }

    return (best_eval, best_move)
}