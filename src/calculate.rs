use chess::{Board, Color, BoardStatus, MoveGen, ChessMove};

pub fn find_best_move(board: &Board, depth: u8) -> (i32, ChessMove) {
    let maximizing = board.side_to_move() == Color::White;
    let (eval, best_move) = calculate(board, depth, i32::MIN, i32::MAX, maximizing);
    println!("Evaluation: {}, Best Move: {}", eval, best_move);
    (eval, best_move)
}

fn calculate(board: &Board, depth: u8, mut alpha: i32, mut beta: i32, maximizing_player: bool) -> (i32, ChessMove) {
    if board.status() == BoardStatus::Checkmate {
        return (if maximizing_player { i32::MIN } else { i32::MAX }, ChessMove::default());
    }

    if depth == 0 {
        return (evaluate(board), ChessMove::default());
    }

    let mut best_score = if maximizing_player { i32::MIN } else { i32::MAX };
    let mut best_move = ChessMove::default();

    for mv in MoveGen::new_legal(&board) {
        let new_board = board.make_move_new(mv);
        let (score, _) = calculate(&new_board, depth - 1, alpha, beta, !maximizing_player);

        if maximizing_player {
            if score > best_score {
                best_score = score;
                best_move = mv;
            }
            if best_score > alpha {
                alpha = best_score;
            }
        } else {
            if score < best_score {
                best_score = score;
                best_move = mv;
            }
            if best_score < beta {
                beta = best_score;
            }
        }

        if alpha >= beta {
            break; // Beta cutoff
        }
    }

    (best_score, best_move)
}

fn evaluate(_board: &Board) -> i32 {
    return 0;
}