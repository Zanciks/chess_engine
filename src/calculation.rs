use std::cmp::max;

use chess::{Board, BoardStatus, ChessMove, Color, MoveGen};
use crate::evaluation::evaluate;

pub fn find_best_move(board: Board, depth: u8) -> (i32, ChessMove) {
    let (mut eval, best_move) = negamax(board, depth, -i32::MAX, i32::MAX);
    if board.side_to_move() == Color::Black {eval *= -1} // we use negamax, so have to negate for the user

    return (eval, best_move)
}

pub fn negamax(board: Board, depth: u8, mut alpha: i32, beta: i32) -> (i32, ChessMove) {
    if board.status() == BoardStatus::Checkmate {return (-i32::MAX, ChessMove::default())}

    if depth == 0 {return (evaluate(board), ChessMove::default())}

    let mut best_eval = -i32::MAX;
    let mut best_move = ChessMove::default();

    for mv in MoveGen::new_legal(&board) {
        let new_board = board.make_move_new(mv);
        let eval = -negamax(new_board, depth - 1, -beta, -alpha).0;
        
        if eval > best_eval {
            best_eval = eval;
            best_move = mv;
        }
        
        alpha = max(alpha, best_eval);
        if alpha >= beta {break}

    }
    return (best_eval, best_move)
}