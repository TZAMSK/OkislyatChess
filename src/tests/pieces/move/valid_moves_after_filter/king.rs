#[cfg(test)]
mod tests {
    use crate::pieces::{Color, Kind, Piece, Position, ValidMove};

    #[test]
    fn king_valid_moves_initialy() {
        let king = Piece::new(Kind::King, Color::Black, Position { x: 3, y: 2 });

        assert_eq!(king.valid_moves.len(), 8);
    }

    #[test]
    fn king_valid_moves_list_initialy() {
        let king = Piece::new(Kind::King, Color::Black, Position { x: 3, y: 2 });

        assert_eq!(
            king.valid_moves,
            vec![
                ValidMove { x: 2, y: 3 },
                ValidMove { x: 3, y: 3 },
                ValidMove { x: 4, y: 3 },
                ValidMove { x: 2, y: 2 },
                ValidMove { x: 4, y: 2 },
                ValidMove { x: 2, y: 1 },
                ValidMove { x: 3, y: 1 },
                ValidMove { x: 4, y: 1 }
            ]
        );
    }

    #[test]
    fn king_valid_moves_after_normal_move() {
        let mut king = Piece::new(Kind::King, Color::Black, Position { x: 2, y: 2 });

        king.make_a_move(3, 2);

        assert_eq!(king.valid_moves.len(), 8);
    }

    #[test]
    fn king_valid_moves_list_after_normal_move() {
        let mut king = Piece::new(Kind::King, Color::Black, Position { x: 2, y: 2 });

        king.make_a_move(3, 2);

        assert_eq!(
            king.valid_moves,
            vec![
                ValidMove { x: 2, y: 3 },
                ValidMove { x: 3, y: 3 },
                ValidMove { x: 4, y: 3 },
                ValidMove { x: 2, y: 2 },
                ValidMove { x: 4, y: 2 },
                ValidMove { x: 2, y: 1 },
                ValidMove { x: 3, y: 1 },
                ValidMove { x: 4, y: 1 }
            ]
        );
    }

    #[test]
    fn king_valid_moves_initialy_can_only_move_east_south_and_south_east_when_placed_top_left_corner(
    ) {
        let king = Piece::new(Kind::King, Color::Black, Position { x: 1, y: 8 });

        assert_eq!(
            king.valid_moves,
            vec![
                ValidMove { x: 2, y: 8 }, // East
                ValidMove { x: 1, y: 7 }, // South
                ValidMove { x: 2, y: 7 }, // South-East
            ]
        );
    }

    #[test]
    fn king_valid_moves_initialy_can_only_move_west_south_west_and_south_when_placed_top_left_corner(
    ) {
        let king = Piece::new(Kind::King, Color::Black, Position { x: 8, y: 8 });

        assert_eq!(
            king.valid_moves,
            vec![
                ValidMove { x: 7, y: 8 }, // West
                ValidMove { x: 7, y: 7 }, // South-West
                ValidMove { x: 8, y: 7 }, // South
            ]
        );
    }

    #[test]
    fn king_valid_moves_initialy_can_only_move_north_north_east_and_east_when_placed_bottom_left_corner(
    ) {
        let king = Piece::new(Kind::King, Color::Black, Position { x: 1, y: 1 });

        assert_eq!(
            king.valid_moves,
            vec![
                ValidMove { x: 1, y: 2 }, // North
                ValidMove { x: 2, y: 2 }, // North-East
                ValidMove { x: 2, y: 1 }, // East
            ]
        );
    }

    #[test]
    fn king_valid_moves_initialy_can_only_move_north_west_north_and_west_when_placed_top_left_corner(
    ) {
        let king = Piece::new(Kind::King, Color::Black, Position { x: 8, y: 1 });

        assert_eq!(
            king.valid_moves,
            vec![
                ValidMove { x: 7, y: 2 }, // North-West
                ValidMove { x: 8, y: 2 }, // North
                ValidMove { x: 7, y: 1 }, // West
            ]
        );
    }
}
