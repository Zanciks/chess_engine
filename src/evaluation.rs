use shakmaty::{Square, board::Board, Color, Role};

pub fn evaluate(board: &Board) -> i32 {
    let mut evaluation = 0;
    for square in Square::ALL {
        if let Some(piece) = board.piece_at(square) {
            evaluation += match (piece.color, piece.role) {
                (Color::White, Role::Pawn)   => 100,
                (Color::White, Role::Knight) => 300,
                (Color::White, Role::Bishop) => 300,
                (Color::White, Role::Rook)   => 500,
                (Color::White, Role::Queen)  => 900,
                (Color::Black, Role::Pawn)   => -100,
                (Color::Black, Role::Knight) => -300,
                (Color::Black, Role::Bishop) => -300,
                (Color::Black, Role::Rook)   => -500,
                (Color::Black, Role::Queen)  => -900,
                _ => 0,
            }
        }
    }
    return evaluation;
}