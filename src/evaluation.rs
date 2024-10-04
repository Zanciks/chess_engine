use shakmaty::{Square, board::Board, Color, Role};
use crate::piece_square_tables::{WHITE_PAWNS, BLACK_PAWNS, WHITE_KNIGHTS, BLACK_KNIGHTS, WHITE_BISHOPS, BLACK_BISHOPS, WHITE_ROOKS, BLACK_ROOKS, WHITE_QUEENS, BLACK_QUEENS};

pub fn evaluate(board: &Board) -> i32 {
    let mut evaluation = 0;
    for square in Square::ALL {
        if let Some(piece) = board.piece_at(square) {
            let square: usize = square.into();
            evaluation += match (piece.color, piece.role) {
                (Color::White, Role::Pawn)   => WHITE_PAWNS[square],
                (Color::White, Role::Knight) => WHITE_KNIGHTS[square],
                (Color::White, Role::Bishop) => WHITE_BISHOPS[square],
                (Color::White, Role::Rook)   => WHITE_ROOKS[square],
                (Color::White, Role::Queen)  => WHITE_QUEENS[square],
                (Color::Black, Role::Pawn)   => BLACK_PAWNS[square],
                (Color::Black, Role::Knight) => BLACK_KNIGHTS[square],
                (Color::Black, Role::Bishop) => BLACK_BISHOPS[square],
                (Color::Black, Role::Rook)   => BLACK_ROOKS[square],
                (Color::Black, Role::Queen)  => BLACK_QUEENS[square],
                _ => 0,
            }
        }
    }
    return evaluation;
}