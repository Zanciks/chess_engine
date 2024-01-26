use chess::{Board, Color, Piece, ALL_SQUARES};

pub fn evaluate(board: Board) -> i32 {
    let mut eval: i32 = 0;
    for square in ALL_SQUARES {
        if let (Some(piece), Some(color)) = (board.piece_on(square), board.color_on(square)) {
            eval += match (piece, color) {
                (Piece::Pawn,   Color::White) => 100,
                (Piece::Knight, Color::White) => 300,
                (Piece::Bishop, Color::White) => 300,
                (Piece::Rook,   Color::White) => 500,
                (Piece::Queen,  Color::White) => 900,

                (Piece::Pawn,   Color::Black) => -100,
                (Piece::Knight, Color::Black) => -300,
                (Piece::Bishop, Color::Black) => -300,
                (Piece::Rook,   Color::Black) => -500,
                (Piece::Queen,  Color::Black) => -900,

                _ => 0,
            }
        }
    }
    if board.side_to_move() == Color::Black {eval *= -1}
    eval
}