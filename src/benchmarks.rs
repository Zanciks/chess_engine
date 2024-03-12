#![feature(test)]
mod tests {
    use test::Bencher;
    use std::str::FromStr;
    use chess::{Board, ChessMove};
    use crate::calculation::find_best_move;

    #[bench]
    fn mate_in_one() {
        // https://lichess.org/XyPA3GUZ/white#28
        let board = Board::from_str("r1b2k1r/ppppqnp1/2n5/6pQ/4N3/2N5/PP3RPP/R5K1 w - - 0 15").unwrap();
        let (eval, mv) = find_best_move(board, 1);
        println!("{} | {}", eval, mv);
        assert_eq!(eval, 2147483647);
        assert_eq!(mv, ChessMove::from_str("a1a8").unwrap());

        // https://lichess.org/EygAm9YU/black#111
        let board = Board::from_str("8/6r1/8/5k1K/5p2/5RP1/5P2/8 b - - 0 56").unwrap();
        let (eval, mv) = find_best_move(board, 1);
        assert_eq!(eval, -2147483647);
        assert_eq!(mv, ChessMove::from_str("g7h7").unwrap());
    }

    #[bench]
    fn mate_in_two() {
        // https://lichess.org/ofEcCPyR/white#50
        let board = Board::from_str("r7/ppp1b1pk/6Np/3P2q1/1P4b1/P2Q2P1/7P/5RK1 w - - 0 26").unwrap();
        let (eval, mv) = find_best_move(board, 3);
        assert_eq!(eval, 2147483647);
        assert_eq!(mv, ChessMove::from_str("g6f8").unwrap());

        // https://lichess.org/DNvSxD8b/black#53
        let board = Board::from_str("2k1r3/1p1R4/p4p2/5R1p/P7/1QP2N2/1PK2n2/5q2 b - - 0 27").unwrap();
        let (eval, mv) = find_best_move(board, 3);
        assert_eq!(eval, -2147483647);
        assert_eq!(mv, ChessMove::from_str("e8e2").unwrap());
    }

    #[bench]
    fn mate_in_three() {
        // https://lichess.org/k1xYI03L/white#84
        let board = Board::from_str("4R3/1p3p1k/p6p/P4qr1/1P1Q4/6P1/5P1K/8 w - - 2 43").unwrap();
        let (eval, mv) = find_best_move(board, 5);
        assert_eq!(eval, 2147483647);
        assert_eq!(mv, ChessMove::from_str("e8h8").unwrap());

        // https://lichess.org/5hnjSaBo/black#71
        let board = Board::from_str("6Q1/4kp2/N3p3/1R1p3p/2nPq3/2P3P1/5P1P/5K2 b - - 6 36").unwrap();
        let (eval, mv) = find_best_move(board, 5);
        assert_eq!(eval, -2147483647);
        assert_eq!(mv, ChessMove::from_str("c4d2").unwrap());
    }
}