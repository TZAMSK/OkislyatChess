#[cfg(test)]
mod tests {
    use crate::board::init_board;
    use crate::pieces::{Color, Kind};
    use crate::tests::board::count_piece_by_type;

    #[test]
    fn board_has_64_cells() {
        let board = init_board();
        let flattened_board: Vec<_> = board.iter().flatten().collect();

        assert_eq!(flattened_board.len(), 64);
    }

    #[test]
    fn board_has_16_pawns() {
        let board = init_board();

        assert_eq!(count_piece_by_type(&board, Kind::Pawn, None).len(), 16);
    }

    #[test]
    fn board_has_8_black_pawns() {
        let board = init_board();

        assert_eq!(
            count_piece_by_type(&board, Kind::Pawn, Some(Color::Black)).len(),
            8
        );
    }

    #[test]
    fn board_has_8_white_pawns() {
        let board = init_board();

        assert_eq!(
            count_piece_by_type(&board, Kind::Pawn, Some(Color::White)).len(),
            8
        );
    }

    #[test]
    fn board_has_4_rooks() {
        let board = init_board();

        assert_eq!(count_piece_by_type(&board, Kind::Rook, None).len(), 4);
    }

    #[test]
    fn board_has_2_black_rooks() {
        let board = init_board();

        assert_eq!(
            count_piece_by_type(&board, Kind::Rook, Some(Color::Black)).len(),
            2
        );
    }

    #[test]
    fn board_has_2_white_rooks() {
        let board = init_board();

        assert_eq!(
            count_piece_by_type(&board, Kind::Rook, Some(Color::White)).len(),
            2
        );
    }

    #[test]
    fn board_has_4_knights() {
        let board = init_board();

        assert_eq!(count_piece_by_type(&board, Kind::Knight, None).len(), 4);
    }

    #[test]
    fn board_has_2_black_knights() {
        let board = init_board();

        assert_eq!(
            count_piece_by_type(&board, Kind::Knight, Some(Color::Black)).len(),
            2
        );
    }

    #[test]
    fn board_has_2_white_knights() {
        let board = init_board();

        assert_eq!(
            count_piece_by_type(&board, Kind::Knight, Some(Color::White)).len(),
            2
        );
    }

    #[test]
    fn board_has_4_bishops() {
        let board = init_board();

        assert_eq!(count_piece_by_type(&board, Kind::Bishop, None).len(), 4);
    }

    #[test]
    fn board_has_2_black_bishops() {
        let board = init_board();

        assert_eq!(
            count_piece_by_type(&board, Kind::Bishop, Some(Color::Black)).len(),
            2
        );
    }

    #[test]
    fn board_has_2_white_bishops() {
        let board = init_board();

        assert_eq!(
            count_piece_by_type(&board, Kind::Bishop, Some(Color::White)).len(),
            2
        );
    }

    #[test]
    fn board_has_2_queens() {
        let board = init_board();

        assert_eq!(count_piece_by_type(&board, Kind::Queen, None).len(), 2);
    }

    #[test]
    fn board_has_1_black_queen() {
        let board = init_board();

        assert_eq!(
            count_piece_by_type(&board, Kind::Queen, Some(Color::Black)).len(),
            1
        );
    }

    #[test]
    fn board_has_1_white_queen() {
        let board = init_board();

        assert_eq!(
            count_piece_by_type(&board, Kind::Queen, Some(Color::White)).len(),
            1
        );
    }

    #[test]
    fn board_has_2_kings() {
        let board = init_board();

        assert_eq!(count_piece_by_type(&board, Kind::King, None).len(), 2);
    }

    #[test]
    fn board_has_1_black_king() {
        let board = init_board();

        assert_eq!(
            count_piece_by_type(&board, Kind::King, Some(Color::Black)).len(),
            1
        );
    }

    #[test]
    fn board_has_1_white_king() {
        let board = init_board();

        assert_eq!(
            count_piece_by_type(&board, Kind::King, Some(Color::White)).len(),
            1
        );
    }
}
