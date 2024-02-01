use chess::{Board, BoardStatus, ChessMove, Color, MoveGen};
use crate::evaluation::evaluate;

pub fn find_best_move(board: Board, depth: u8) -> (i32, ChessMove) {
    return minimax(board, depth, -i32::MAX, i32::MAX);
}

fn minimax(board: Board, depth: u8, mut alpha: i32, mut beta: i32) -> (i32, ChessMove) {
    if board.status() == BoardStatus::Checkmate {
        if board.side_to_move() == Color::White {return (-i32::MAX, ChessMove::default())}
        else {return (i32::MAX, ChessMove::default())}
    }

    if depth == 0 {return (evaluate(board), ChessMove::default())}

    let mut best_eval = if board.side_to_move() == Color::White {-i32::MAX} else {i32::MAX};
    let mut best_move = ChessMove::default();

    for mv in MoveGen::new_legal(&board) {
        let new_board = board.make_move_new(mv);
        let eval = minimax(new_board, depth - 1, alpha, beta).0;
        match board.side_to_move() {
            Color::White => {
                if eval > best_eval {
                    best_eval = eval;
                    best_move = mv;
                }
                if best_eval > alpha {
                    alpha = best_eval;
                }
            },
            Color::Black => {
                if eval < best_eval {
                    best_eval = eval;
                    best_move = mv;
                }
                if best_eval < beta {
                    beta = best_eval
                }
            }
        }
        if alpha >= beta {
            break; // beta cutoff
        }
    }
    
    return (best_eval, best_move)
}