use chess::{Board, Color, BoardStatus, MoveGen, ChessMove, ALL_SQUARES, Piece};

pub fn find_best_move(board: &Board, depth: u8, nodes: &mut u32) -> (i32, ChessMove, u32) {
    let maximizing = board.side_to_move() == Color::White;
    let (eval, best_move) = calculate(board, depth, i32::MIN, i32::MAX, maximizing, nodes);
    (eval, best_move, *nodes)
}

fn calculate(board: &Board, depth: u8, mut alpha: i32, mut beta: i32, maximizing_player: bool, nodes: &mut u32) -> (i32, ChessMove) {
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
        let (score, _) = calculate(&new_board, depth - 1, alpha, beta, !maximizing_player, nodes);
        *nodes += 1;
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

fn evaluate(board: &Board) -> i32 {
    let mut evaluation: i32 = 0;
    for square in ALL_SQUARES {
        match (board.piece_on(square), board.color_on(square)) {
            (Some(Piece::Pawn),   Some(Color::White)) => evaluation += 100,
            (Some(Piece::Knight), Some(Color::White)) => evaluation += 300,
            (Some(Piece::Bishop), Some(Color::White)) => evaluation += 300,
            (Some(Piece::Rook),   Some(Color::White)) => evaluation += 500,
            (Some(Piece::Queen),  Some(Color::White)) => evaluation += 900,
            (Some(Piece::Pawn),   Some(Color::Black)) => evaluation -= 100,
            (Some(Piece::Knight), Some(Color::Black)) => evaluation -= 300,
            (Some(Piece::Bishop), Some(Color::Black)) => evaluation -= 300,
            (Some(Piece::Rook),   Some(Color::Black)) => evaluation -= 500,
            (Some(Piece::Queen),  Some(Color::Black)) => evaluation -= 900,
            _ => ()
        }
    }

    return evaluation;
}