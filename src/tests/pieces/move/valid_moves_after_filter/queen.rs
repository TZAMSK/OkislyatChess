#[cfg(test)]
mod tests {
    use crate::pieces::{Color, Kind, Piece, Position, ValidMove};

    #[test]
    fn queen_valid_moves_initialy() {
        let queen = Piece::new(Kind::Queen, Color::Black, Position { x: 5, y: 5 });

        assert_eq!(queen.valid_moves.len(), 27);
    }

    #[test]
    fn queen_valid_moves_list_initialy() {
        let queen = Piece::new(Kind::Queen, Color::Black, Position { x: 5, y: 5 });

        assert_eq!(
            queen.valid_moves,
            vec![
                ValidMove { x: 4, y: 5 }, // West
                ValidMove { x: 3, y: 5 },
                ValidMove { x: 2, y: 5 },
                ValidMove { x: 1, y: 5 },
                ValidMove { x: 4, y: 6 }, // North-West
                ValidMove { x: 3, y: 7 },
                ValidMove { x: 2, y: 8 },
                ValidMove { x: 5, y: 6 }, // North
                ValidMove { x: 5, y: 7 },
                ValidMove { x: 5, y: 8 },
                ValidMove { x: 6, y: 6 }, // North-East
                ValidMove { x: 7, y: 7 },
                ValidMove { x: 8, y: 8 },
                ValidMove { x: 6, y: 5 }, // East
                ValidMove { x: 7, y: 5 },
                ValidMove { x: 8, y: 5 },
                ValidMove { x: 6, y: 4 }, // South-East
                ValidMove { x: 7, y: 3 },
                ValidMove { x: 8, y: 2 },
                ValidMove { x: 5, y: 4 }, // South
                ValidMove { x: 5, y: 3 },
                ValidMove { x: 5, y: 2 },
                ValidMove { x: 5, y: 1 },
                ValidMove { x: 4, y: 4 }, // South-West
                ValidMove { x: 3, y: 3 },
                ValidMove { x: 2, y: 2 },
                ValidMove { x: 1, y: 1 },
            ]
        );
    }

    #[test]
    fn queen_valid_moves_after_move() {
        let mut queen = Piece::new(Kind::Queen, Color::Black, Position { x: 4, y: 5 });

        queen.make_a_move(5, 5);

        assert_eq!(queen.valid_moves.len(), 27);
    }

    #[test]
    fn queen_valid_moves_list_after_move() {
        let mut queen = Piece::new(Kind::Queen, Color::Black, Position { x: 4, y: 5 });

        queen.make_a_move(5, 5);

        assert_eq!(
            queen.valid_moves,
            vec![
                ValidMove { x: 4, y: 5 }, // West
                ValidMove { x: 3, y: 5 },
                ValidMove { x: 2, y: 5 },
                ValidMove { x: 1, y: 5 },
                ValidMove { x: 4, y: 6 }, // North-West
                ValidMove { x: 3, y: 7 },
                ValidMove { x: 2, y: 8 },
                ValidMove { x: 5, y: 6 }, // North
                ValidMove { x: 5, y: 7 },
                ValidMove { x: 5, y: 8 },
                ValidMove { x: 6, y: 6 }, // North-East
                ValidMove { x: 7, y: 7 },
                ValidMove { x: 8, y: 8 },
                ValidMove { x: 6, y: 5 }, // East
                ValidMove { x: 7, y: 5 },
                ValidMove { x: 8, y: 5 },
                ValidMove { x: 6, y: 4 }, // South-East
                ValidMove { x: 7, y: 3 },
                ValidMove { x: 8, y: 2 },
                ValidMove { x: 5, y: 4 }, // South
                ValidMove { x: 5, y: 3 },
                ValidMove { x: 5, y: 2 },
                ValidMove { x: 5, y: 1 },
                ValidMove { x: 4, y: 4 }, // South-West
                ValidMove { x: 3, y: 3 },
                ValidMove { x: 2, y: 2 },
                ValidMove { x: 1, y: 1 },
            ]
        );
    }

    #[test]
    fn queen_valid_moves_initialy_can_only_move_east_south_east_and_south_when_placed_top_left_corner(
    ) {
        let queen = Piece::new(Kind::Queen, Color::Black, Position { x: 1, y: 8 });

        assert_eq!(
            queen.valid_moves,
            vec![
                ValidMove { x: 2, y: 8 }, // East
                ValidMove { x: 3, y: 8 },
                ValidMove { x: 4, y: 8 },
                ValidMove { x: 5, y: 8 },
                ValidMove { x: 6, y: 8 },
                ValidMove { x: 7, y: 8 },
                ValidMove { x: 8, y: 8 },
                ValidMove { x: 2, y: 7 }, // South-East
                ValidMove { x: 3, y: 6 },
                ValidMove { x: 4, y: 5 },
                ValidMove { x: 5, y: 4 },
                ValidMove { x: 6, y: 3 },
                ValidMove { x: 7, y: 2 },
                ValidMove { x: 8, y: 1 },
                ValidMove { x: 1, y: 7 }, // South
                ValidMove { x: 1, y: 6 },
                ValidMove { x: 1, y: 5 },
                ValidMove { x: 1, y: 4 },
                ValidMove { x: 1, y: 3 },
                ValidMove { x: 1, y: 2 },
                ValidMove { x: 1, y: 1 },
            ]
        );
    }

    #[test]
    fn queen_valid_moves_initialy_can_only_move_west_south_and_south_west_when_placed_top_right_corner(
    ) {
        let queen = Piece::new(Kind::Queen, Color::Black, Position { x: 8, y: 8 });

        assert_eq!(
            queen.valid_moves,
            vec![
                ValidMove { x: 7, y: 8 }, // West
                ValidMove { x: 6, y: 8 },
                ValidMove { x: 5, y: 8 },
                ValidMove { x: 4, y: 8 },
                ValidMove { x: 3, y: 8 },
                ValidMove { x: 2, y: 8 },
                ValidMove { x: 1, y: 8 },
                ValidMove { x: 8, y: 7 }, // South
                ValidMove { x: 8, y: 6 },
                ValidMove { x: 8, y: 5 },
                ValidMove { x: 8, y: 4 },
                ValidMove { x: 8, y: 3 },
                ValidMove { x: 8, y: 2 },
                ValidMove { x: 8, y: 1 },
                ValidMove { x: 7, y: 7 }, // South-West
                ValidMove { x: 6, y: 6 },
                ValidMove { x: 5, y: 5 },
                ValidMove { x: 4, y: 4 },
                ValidMove { x: 3, y: 3 },
                ValidMove { x: 2, y: 2 },
                ValidMove { x: 1, y: 1 },
            ]
        );
    }

    #[test]
    fn queen_valid_moves_initialy_can_only_move_north_north_east_and_east_when_placed_bottom_left_corner(
    ) {
        let queen = Piece::new(Kind::Queen, Color::Black, Position { x: 1, y: 1 });

        assert_eq!(
            queen.valid_moves,
            vec![
                ValidMove { x: 1, y: 2 }, // North
                ValidMove { x: 1, y: 3 },
                ValidMove { x: 1, y: 4 },
                ValidMove { x: 1, y: 5 },
                ValidMove { x: 1, y: 6 },
                ValidMove { x: 1, y: 7 },
                ValidMove { x: 1, y: 8 },
                ValidMove { x: 2, y: 2 }, // North-East
                ValidMove { x: 3, y: 3 },
                ValidMove { x: 4, y: 4 },
                ValidMove { x: 5, y: 5 },
                ValidMove { x: 6, y: 6 },
                ValidMove { x: 7, y: 7 },
                ValidMove { x: 8, y: 8 },
                ValidMove { x: 2, y: 1 }, // East
                ValidMove { x: 3, y: 1 },
                ValidMove { x: 4, y: 1 },
                ValidMove { x: 5, y: 1 },
                ValidMove { x: 6, y: 1 },
                ValidMove { x: 7, y: 1 },
                ValidMove { x: 8, y: 1 },
            ]
        );
    }

    #[test]
    fn queen_valid_moves_initialy_can_only_move_west_north_west_and_north_when_placed_bottom_right_corner(
    ) {
        let queen = Piece::new(Kind::Queen, Color::Black, Position { x: 8, y: 1 });

        assert_eq!(
            queen.valid_moves,
            vec![
                ValidMove { x: 7, y: 1 }, // West
                ValidMove { x: 6, y: 1 },
                ValidMove { x: 5, y: 1 },
                ValidMove { x: 4, y: 1 },
                ValidMove { x: 3, y: 1 },
                ValidMove { x: 2, y: 1 },
                ValidMove { x: 1, y: 1 },
                ValidMove { x: 7, y: 2 }, // North-West
                ValidMove { x: 6, y: 3 },
                ValidMove { x: 5, y: 4 },
                ValidMove { x: 4, y: 5 },
                ValidMove { x: 3, y: 6 },
                ValidMove { x: 2, y: 7 },
                ValidMove { x: 1, y: 8 },
                ValidMove { x: 8, y: 2 }, // North
                ValidMove { x: 8, y: 3 },
                ValidMove { x: 8, y: 4 },
                ValidMove { x: 8, y: 5 },
                ValidMove { x: 8, y: 6 },
                ValidMove { x: 8, y: 7 },
                ValidMove { x: 8, y: 8 },
            ]
        );
    }
}
