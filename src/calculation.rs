use std::cmp::max;

use chess::{Board, BoardStatus, ChessMove, Color, MoveGen, Piece};
use crate::evaluation::evaluate;

pub fn find_best_move(board: Board, depth: u8) -> (i32, ChessMove) {
    let (mut eval, best_move) = negamax(board, depth, -i32::MAX, i32::MAX);
    if board.side_to_move() == Color::Black {eval *= -1} // we use negamax, so have to negate for the user

    return (eval, best_move)
}

fn negamax(board: Board, depth: u8, mut alpha: i32, beta: i32) -> (i32, ChessMove) {
    if board.status() == BoardStatus::Checkmate {return (-i32::MAX, ChessMove::default())}

    if depth == 0 {return (evaluate(board), ChessMove::default())}

    let mut best_eval = -i32::MAX;
    let mut best_move = ChessMove::default();

    for mv in sort_moves(&board) {
        let new_board = board.make_move_new(mv);
        let eval = -negamax(new_board, depth - 1, -beta, -alpha).0;
        
        if eval > best_eval {
            best_eval = eval;
            best_move = mv;
        }
        
        alpha = max(alpha, best_eval);
        if alpha >= beta {break}
    }

    (best_eval, best_move)
}

pub fn sort_moves(board: &Board) -> Vec<ChessMove> {
    let mut vector: Vec<(ChessMove, i32)> = vec![];
    for mv in MoveGen::new_legal(board) {
        if let Some(dest_piece) = board.piece_on(mv.get_dest()) {
            let dest_value = piece_to_i32(dest_piece);
            vector.push((mv, dest_value));
        } else {
            vector.push((mv, 0));
        }
    }

    vector.sort_by_key(|item| item.1);
    vector.reverse();

    let output = vector.into_iter().map(|item| item.0).collect();
    output
}

pub fn piece_to_i32(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 100,
        Piece::Knight => 300,
        Piece::Bishop => 300,
        Piece::Rook => 500,
        Piece::Queen => 900,
        _ => 0
    }
}