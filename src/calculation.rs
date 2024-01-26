#![allow(dead_code, unused_variables)]

use chess::{Board, BoardStatus, ChessMove, MoveGen};

use crate::evaluation::evaluate;

pub fn calculate(board: Board) -> (i32, ChessMove) {
    let mut best_eval = i32::MIN;
    let mut best_move = ChessMove::default();

    for mv in MoveGen::new_legal(&board) {
        let new_board = board.make_move_new(mv);
        let eval = -minimax(new_board, 2, best_eval);
        if eval > best_eval {
            best_eval = eval;
            best_move = mv;
        }
    }
    return (best_eval, best_move);
}

pub fn minimax(board: Board, depth: u8, mut best_eval: i32) -> i32 {
    if board.status() == BoardStatus::Checkmate {return -i32::MAX } // we can negate MAX, but we CANT negate MIN, so we just use -MAX lol
    if depth == 0 {return evaluate(board)}

    for mv in MoveGen::new_legal(&board) {
        let new_board = board.make_move_new(mv);
        let eval = -minimax(new_board, depth - 1, best_eval);
        if eval > best_eval {best_eval = eval}
    }

    return best_eval;
}