#[cfg(test)]
mod tests {
    use crate::board::{get_piece, init_board};
    use crate::pieces::{Color, Kind, Piece, Position, ValidMove};
    use crate::tests::board::count_piece_by_type;

    #[test]
    fn board_has_a_white_pawn_at_position_1_2() {
        let board = init_board();

        assert_eq!(
            get_piece(&board, 1, 2),
            Some(&Piece {
                kind: Kind::Pawn,
                color: Color::White,
                position: Position { x: 1, y: 2 },
                valid_moves: vec![
                    ValidMove { x: 0, y: 3 },
                    ValidMove { x: 1, y: 3 },
                    ValidMove { x: 2, y: 3 },
                    ValidMove { x: 1, y: 4 }
                ]
            })
        );
    }

    #[test]
    fn board_has_a_black_rook_at_position_1_8() {
        let board = init_board();

        assert_eq!(
            get_piece(&board, 1, 8),
            Some(&Piece {
                kind: Kind::Rook,
                color: Color::Black,
                position: Position { x: 1, y: 8 },
                valid_moves: vec![
                    ValidMove { x: 2, y: 8 }, // East
                    ValidMove { x: 3, y: 8 },
                    ValidMove { x: 4, y: 8 },
                    ValidMove { x: 5, y: 8 },
                    ValidMove { x: 6, y: 8 },
                    ValidMove { x: 7, y: 8 },
                    ValidMove { x: 8, y: 8 },
                    ValidMove { x: 1, y: 7 }, // South
                    ValidMove { x: 1, y: 6 },
                    ValidMove { x: 1, y: 5 },
                    ValidMove { x: 1, y: 4 },
                    ValidMove { x: 1, y: 3 },
                    ValidMove { x: 1, y: 2 },
                    ValidMove { x: 1, y: 1 },
                ]
            })
        );
    }

    #[test]
    fn board_has_a_white_knight_at_position_2_1() {
        let board = init_board();

        assert_eq!(
            get_piece(&board, 2, 1),
            Some(&Piece {
                kind: Kind::Knight,
                color: Color::White,
                position: Position { x: 2, y: 1 },
                valid_moves: vec![
                    ValidMove { x: 1, y: 3 },
                    ValidMove { x: 3, y: 3 },
                    ValidMove { x: 4, y: 2 },
                ]
            })
        );
    }

    #[test]
    fn board_has_a_black_bishop_at_position_3_8() {
        let board = init_board();

        assert_eq!(
            get_piece(&board, 3, 8),
            Some(&Piece {
                kind: Kind::Bishop,
                color: Color::Black,
                position: Position { x: 3, y: 8 },
                valid_moves: vec![
                    ValidMove { x: 2, y: 7 }, // South-West
                    ValidMove { x: 1, y: 6 },
                    ValidMove { x: 4, y: 7 }, // South-East
                    ValidMove { x: 5, y: 6 },
                    ValidMove { x: 6, y: 5 },
                    ValidMove { x: 7, y: 4 },
                    ValidMove { x: 8, y: 3 },
                ]
            })
        );
    }

    #[test]
    fn board_has_a_white_queen_at_position_4_1() {
        let board = init_board();

        assert_eq!(
            get_piece(&board, 4, 1),
            Some(&Piece {
                kind: Kind::Queen,
                color: Color::White,
                position: Position { x: 4, y: 1 },
                valid_moves: vec![
                    ValidMove { x: 3, y: 1 }, // West
                    ValidMove { x: 2, y: 1 },
                    ValidMove { x: 1, y: 1 },
                    ValidMove { x: 3, y: 2 }, // North-West
                    ValidMove { x: 2, y: 3 },
                    ValidMove { x: 1, y: 4 },
                    ValidMove { x: 4, y: 2 }, // North
                    ValidMove { x: 4, y: 3 },
                    ValidMove { x: 4, y: 4 },
                    ValidMove { x: 4, y: 5 },
                    ValidMove { x: 4, y: 6 },
                    ValidMove { x: 4, y: 7 },
                    ValidMove { x: 4, y: 8 },
                    ValidMove { x: 5, y: 2 }, // North-Est
                    ValidMove { x: 6, y: 3 },
                    ValidMove { x: 7, y: 4 },
                    ValidMove { x: 8, y: 5 },
                    ValidMove { x: 5, y: 1 }, // East
                    ValidMove { x: 6, y: 1 },
                    ValidMove { x: 7, y: 1 },
                    ValidMove { x: 8, y: 1 },
                ]
            })
        );
    }

    #[test]
    fn board_has_a_black_king_at_position_5_8() {
        let board = init_board();

        assert_eq!(
            get_piece(&board, 5, 8),
            Some(&Piece {
                kind: Kind::King,
                color: Color::Black,
                position: Position { x: 5, y: 8 },
                valid_moves: vec![
                    ValidMove { x: 4, y: 8 },
                    ValidMove { x: 6, y: 8 },
                    ValidMove { x: 4, y: 7 },
                    ValidMove { x: 5, y: 7 },
                    ValidMove { x: 6, y: 7 },
                ]
            })
        );
    }
}
