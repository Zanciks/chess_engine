use chess::{Board, Color, Piece, ALL_SQUARES};

pub fn evaluate(board: Board) -> i32 {
    let mut eval = count_pieces(board);
    if board.side_to_move() == Color::White {eval *= -1}
    eval
}

pub fn count_pieces(board: Board) -> i32 {
    let mut sum = 0;

    for square in ALL_SQUARES {
        if let Some(piece) = board.piece_on(square) {
            if let Some(color) = board.color_on(square) {
                sum += match (piece, color) {
                    (Piece::Pawn, Color::White) => 100,
                    _ => 0,
                }
            }
        }
    }

    sum
}