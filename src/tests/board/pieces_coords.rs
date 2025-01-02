#[cfg(test)]
mod tests {
    use crate::board::init_board;
    use crate::pieces::{Color, Kind, Position};
    use crate::tests::board::get_piece_coords;

    #[test]
    fn board_has_16_pawns_on_their_respected_initial_coord() {
        let board = init_board();

        assert_eq!(
            get_piece_coords(&board, Kind::Pawn, None),
            vec![
                Position { x: 1, y: 7 },
                Position { x: 2, y: 7 },
                Position { x: 3, y: 7 },
                Position { x: 4, y: 7 },
                Position { x: 5, y: 7 },
                Position { x: 6, y: 7 },
                Position { x: 7, y: 7 },
                Position { x: 8, y: 7 },
                Position { x: 1, y: 2 },
                Position { x: 2, y: 2 },
                Position { x: 3, y: 2 },
                Position { x: 4, y: 2 },
                Position { x: 5, y: 2 },
                Position { x: 6, y: 2 },
                Position { x: 7, y: 2 },
                Position { x: 8, y: 2 },
            ]
        );
    }

    #[test]
    fn board_has_8_black_pawns_on_their_respected_initial_coord() {
        let board = init_board();

        assert_eq!(
            get_piece_coords(&board, Kind::Pawn, Some(Color::Black)),
            vec![
                Position { x: 1, y: 7 },
                Position { x: 2, y: 7 },
                Position { x: 3, y: 7 },
                Position { x: 4, y: 7 },
                Position { x: 5, y: 7 },
                Position { x: 6, y: 7 },
                Position { x: 7, y: 7 },
                Position { x: 8, y: 7 },
            ]
        );
    }

    #[test]
    fn board_has_8_white_pawns_on_their_respected_initial_coord() {
        let board = init_board();

        assert_eq!(
            get_piece_coords(&board, Kind::Pawn, Some(Color::White)),
            vec![
                Position { x: 1, y: 2 },
                Position { x: 2, y: 2 },
                Position { x: 3, y: 2 },
                Position { x: 4, y: 2 },
                Position { x: 5, y: 2 },
                Position { x: 6, y: 2 },
                Position { x: 7, y: 2 },
                Position { x: 8, y: 2 },
            ]
        );
    }

    #[test]
    fn board_has_4_rooks_on_their_respected_initial_coord() {
        let board = init_board();

        assert_eq!(
            get_piece_coords(&board, Kind::Rook, None),
            vec![
                Position { x: 1, y: 8 },
                Position { x: 8, y: 8 },
                Position { x: 1, y: 1 },
                Position { x: 8, y: 1 },
            ]
        );
    }

    #[test]
    fn board_has_2_black_rooks_on_their_respected_initial_coord() {
        let board = init_board();

        assert_eq!(
            get_piece_coords(&board, Kind::Rook, Some(Color::Black)),
            vec![Position { x: 1, y: 8 }, Position { x: 8, y: 8 },]
        );
    }

    #[test]
    fn board_has_2_white_rooks_on_their_respected_initial_coord() {
        let board = init_board();

        assert_eq!(
            get_piece_coords(&board, Kind::Rook, Some(Color::White)),
            vec![Position { x: 1, y: 1 }, Position { x: 8, y: 1 },]
        );
    }

    #[test]
    fn board_has_4_knights_on_their_respected_initial_coord() {
        let board = init_board();

        assert_eq!(
            get_piece_coords(&board, Kind::Knight, None),
            vec![
                Position { x: 2, y: 8 },
                Position { x: 7, y: 8 },
                Position { x: 2, y: 1 },
                Position { x: 7, y: 1 },
            ]
        );
    }

    #[test]
    fn board_has_2_black_knights_on_their_respected_initial_coord() {
        let board = init_board();

        assert_eq!(
            get_piece_coords(&board, Kind::Knight, Some(Color::Black)),
            vec![Position { x: 2, y: 8 }, Position { x: 7, y: 8 },]
        );
    }

    #[test]
    fn board_has_2_white_knights_on_their_respected_initial_coord() {
        let board = init_board();

        assert_eq!(
            get_piece_coords(&board, Kind::Knight, Some(Color::White)),
            vec![Position { x: 2, y: 1 }, Position { x: 7, y: 1 },]
        );
    }

    #[test]
    fn board_has_4_bishops_on_their_respected_initial_coord() {
        let board = init_board();

        assert_eq!(
            get_piece_coords(&board, Kind::Bishop, None),
            vec![
                Position { x: 3, y: 8 },
                Position { x: 6, y: 8 },
                Position { x: 3, y: 1 },
                Position { x: 6, y: 1 },
            ]
        );
    }

    #[test]
    fn board_has_2_black_bishops_on_their_respected_initial_coord() {
        let board = init_board();

        assert_eq!(
            get_piece_coords(&board, Kind::Bishop, Some(Color::Black)),
            vec![Position { x: 3, y: 8 }, Position { x: 6, y: 8 },]
        );
    }

    #[test]
    fn board_has_2_white_bishops_on_their_respected_initial_coord() {
        let board = init_board();

        assert_eq!(
            get_piece_coords(&board, Kind::Bishop, Some(Color::White)),
            vec![Position { x: 3, y: 1 }, Position { x: 6, y: 1 },]
        );
    }

    #[test]
    fn board_has_2_queens_on_their_respected_initial_coord() {
        let board = init_board();

        assert_eq!(
            get_piece_coords(&board, Kind::Queen, None),
            vec![Position { x: 4, y: 8 }, Position { x: 4, y: 1 }]
        );
    }

    #[test]
    fn board_has_1_black_queen_on_its_respected_initial_coord() {
        let board = init_board();

        assert_eq!(
            get_piece_coords(&board, Kind::Queen, Some(Color::Black)),
            vec![Position { x: 4, y: 8 }]
        );
    }

    #[test]
    fn board_has_2_white_queen_on_its_respected_initial_coord() {
        let board = init_board();

        assert_eq!(
            get_piece_coords(&board, Kind::Queen, Some(Color::White)),
            vec![Position { x: 4, y: 1 }]
        );
    }

    #[test]
    fn board_has_2_kings_on_their_respected_initial_coord() {
        let board = init_board();

        assert_eq!(
            get_piece_coords(&board, Kind::King, None),
            vec![Position { x: 5, y: 8 }, Position { x: 5, y: 1 }]
        );
    }

    #[test]
    fn board_has_1_black_king_on_its_respected_initial_coord() {
        let board = init_board();

        assert_eq!(
            get_piece_coords(&board, Kind::King, Some(Color::Black)),
            vec![Position { x: 5, y: 8 }]
        );
    }

    #[test]
    fn board_has_2_white_king_on_its_respected_initial_coord() {
        let board = init_board();

        assert_eq!(
            get_piece_coords(&board, Kind::King, Some(Color::White)),
            vec![Position { x: 5, y: 1 }]
        );
    }
}
