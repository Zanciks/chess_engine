use chess::{Board, Color, Piece, ALL_SQUARES};

pub fn evaluate(board: Board) -> i32 {
    let (_opening_weight, _middlegame_weight, _endgame_weight) = game_phase_weights(board);

    let mut eval = count_material(board);
    // eval += endgame_push_king_to_corner(board, endgame_weight);

    if board.side_to_move() == Color::Black {eval *= -1}
    eval
}

pub fn count_material(board: Board) -> i32 {
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

fn _endgame_push_king_to_corner(_board: Board, _endgame_weight: i32) -> i32 {
    todo!()
}

// we're going to take in the board and return 3 values from 0 to 100 representing opening, middlegame, endgame
// currently this function won't deal with openings because it's kinda complicated tbh
fn game_phase_weights(board: Board) -> (i32, i32, i32) {
    let piece_count = count_pieces(board);

    let middlegame_weight = piece_count * 100 / 32;
    let endgame_weight = 100 - middlegame_weight;

    (0, middlegame_weight, endgame_weight)
}

fn count_pieces(board: Board) -> i32 {
    let mut sum = 0;
    for square in ALL_SQUARES {
        if let Some(_) = board.piece_on(square) {
            sum += 1
        }
    }
    sum
}