use std::cmp::max;

use chess::{Board, Color, Piece, Square, ALL_SQUARES};

pub fn evaluate(board: Board) -> i32 {
    let endgame_weight = endgame_weight(board);
    let mut eval = count_material(board);
    eval -= force_king_to_corner_endgame(board.king_square(board.side_to_move()), board.king_square(!board.side_to_move()), endgame_weight);
    return eval;
}

fn count_material(board: Board) -> i32 {
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

fn endgame_weight(board: Board) -> i32 {
    let pawns = board.pieces(Piece::Pawn).popcnt();
    let knights = board.pieces(Piece::Knight).popcnt();
    let bishops = board.pieces(Piece::Bishop).popcnt();
    let rooks = board.pieces(Piece::Rook).popcnt();
    let queens = board.pieces(Piece::Queen).popcnt();

    let count = (pawns + knights + bishops + rooks + queens) as f32;

    100 - (count * 10.0 / 3.0) as i32
}

fn force_king_to_corner_endgame(friendly_king: Square, enemy_king: Square, endgame_weight: i32) -> i32 {
    // very much inspired (if not just stolen!) from https://youtu.be/U4ogK0MIzqk?si=7ONAEMHrscMBqbwW&t=1063
    let mut evaluation = 0;

    let enemy_file = enemy_king.get_file().to_index() as i32;
    let enemy_rank = enemy_king.get_rank().to_index() as i32;

    let distance_to_center_file = max(3 - enemy_file, enemy_file - 4);
    let distance_to_center_rank = max(3 - enemy_rank, enemy_rank - 4);

    let distance_to_center = distance_to_center_file + distance_to_center_rank;
    evaluation += distance_to_center;

    let friendly_file = friendly_king.get_file().to_index() as i32;
    let friendly_rank = friendly_king.get_rank().to_index() as i32;

    let file_dist = (friendly_file - enemy_file).abs();
    let rank_dist = (friendly_rank - enemy_rank).abs();

    let dist = file_dist + rank_dist;
    evaluation += 14 - dist;

    evaluation * endgame_weight
}