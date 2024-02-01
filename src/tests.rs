#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use chess::{Board, ChessMove};
    use crate::calculation::find_best_move;

    #[test]
    fn mate_in_one() {
        // https://lichess.org/29OBse7Z/white#44
        let board = Board::from_str("r2q1r1k/1p2Np1p/p1pp2n1/5bQ1/7R/5P2/PPP3PP/R5K1 w - - 0 23").unwrap();
        let (eval, mv) = find_best_move(board, 1);
        assert_eq!(eval, 2147483647);
        assert_eq!(mv, ChessMove::from_str("g5f6").unwrap());

        // https://lichess.org/EygAm9YU/black#111
        let board = Board::from_str("8/6r1/8/5k1K/5p2/5RP1/5P2/8 b - - 0 56").unwrap();
        let (eval, mv) = find_best_move(board, 1);
        assert_eq!(eval, -2147483647);
        assert_eq!(mv, ChessMove::from_str("g7h7").unwrap());
    }

    #[test]
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
}