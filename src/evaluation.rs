use chess::{Board, Color, Piece, ALL_SQUARES};

pub fn evaluate(board: Board) -> i32 {
    let mut eval = count_pieces(board);
    if board.side_to_move() == Color::Black {eval *= -1}
    eval
}

pub fn count_pieces(board: Board) -> i32 {
    let mut sum = 0;

    for square in ALL_SQUARES {
        if let Some(piece) = board.piece_on(square) {
            if let Some(color) = board.color_on(square) {
                sum += match (piece, color) {
                    (Piece::Pawn,   Color::White) =>  100,
                    (Piece::Knight, Color::White) =>  300,
                    (Piece::Bishop, Color::White) =>  300,
                    (Piece::Rook,   Color::White) =>  300,
                    (Piece::Queen,  Color::White) =>  300,
                    (Piece::King,   Color::White) =>  300,

                    (Piece::Pawn,   Color::Black) => -100,
                    (Piece::Knight, Color::Black) => -300,
                    (Piece::Bishop, Color::Black) => -300,
                    (Piece::Rook,   Color::Black) => -300,
                    (Piece::Queen,  Color::Black) => -300,
                    (Piece::King,   Color::Black) => -300,
                }
            }
        }
    }

    sum
}