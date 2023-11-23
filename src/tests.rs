mod mate_tests { // all of these tests are random mate in X puzzles from lichess.org/training
    use::std::str::FromStr;
    use crate::calculate;
    use super::*;
    use chess::{Board, ChessMove, Square};

    #[test]
    fn test_mate_in_1_white() {
        let fen = "r3qk1r/1bpnb2p/p3B3/5p1Q/Pp1P4/4P1N1/1P3PPP/R3K2R w KQ - 1 16";
        let board = Board::from_str(fen).unwrap();
        let (eval, mv) = calculate::find_best_move(&board, 1);
        assert_eq!((eval, mv), (i32::MAX, ChessMove::new(Square::H5, Square::H6, None)));
    }

    #[test]
    fn test_mate_in_1_black() {
        let fen = "5r1k/pp1Q2p1/2p2r1b/8/4Pq2/2NB2RP/PPP1KP2/5R2 b - - 0 25";
        let board = Board::from_str(fen).unwrap();
        let (eval, mv) = calculate::find_best_move(&board, 1);
        assert_eq!((eval, mv), (i32::MIN, ChessMove::new(Square::F4, Square::D2, None)));
    }

    #[test]
    fn test_mate_in_2_white() {
        let fen = "1rb2r2/ppq1np1p/4pNpk/2p1b3/3p4/1B1P4/PPP3PP/R3QRK1 w - - 0 18";
        let board = Board::from_str(fen).unwrap();
        let (eval, mv) = calculate::find_best_move(&board, 3);
        assert_eq!((eval, mv), (i32::MAX, ChessMove::new(Square::E1, Square::H4, None)));
    }

    #[test]
    fn test_mate_in_2_black() {
        let fen = "7r/5k2/2p2p2/1pBbb2p/6rP/2P3P1/PP1N1P1K/R3R3 b - - 4 25";
        let board = Board::from_str(fen).unwrap();
        let (eval, mv) = calculate::find_best_move(&board, 3);
        assert_eq!((eval, mv), (i32::MIN, ChessMove::new(Square::G4, Square::H4, None)));
    }

    #[test]
    fn test_mate_in_3_white() {
        let fen = "r1bq1rk1/3n1pp1/2p1p3/pp1p2b1/3P4/2PBP3/PPQN1PP1/R3K2R w KQ - 0 14";
        let board = Board::from_str(fen).unwrap();
        let (eval, mv) = calculate::find_best_move(&board, 5);
        assert_eq!((eval, mv), (i32::MAX, ChessMove::new(Square::D3, Square::H7, None)));
    }

    #[test]
    fn test_mate_in_3_black() {
        let fen = "6k1/5ppp/1pP1b3/3qP3/pnQ1N2P/P5P1/1P3P2/1K3B1R b - - 2 25";
        let board = Board::from_str(fen).unwrap();
        let (eval, mv) = calculate::find_best_move(&board, 5);
        assert_eq!((eval, mv), (i32::MIN, ChessMove::new(Square::D5, Square::D1, None)));
    }

    #[test]
    fn test_mate_in_4_white() {
        let fen = "1rb1r1kb/p3pp1p/6pB/q1pN4/1p2P1P1/P4n2/NPPQ4/1K1R3R w - - 0 21";
        let board = Board::from_str(fen).unwrap();
        let (eval, mv) = calculate::find_best_move(&board, 7);
        assert_eq!((eval, mv), (i32::MAX, ChessMove::new(Square::D5, Square::E7, None)));
    }

    #[test]
    fn test_mate_in_4_black() {
        let fen = "8/6qk/Q1B1p1pp/1p6/8/1P2P3/P3nPP1/7K b - - 4 39";
        let board = Board::from_str(fen).unwrap();
        let (eval, mv) = calculate::find_best_move(&board, 7);
        assert_eq!((eval, mv), (i32::MIN, ChessMove::new(Square::G7, Square::A1, None)));
    }
}
