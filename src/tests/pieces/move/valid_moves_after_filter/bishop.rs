#[cfg(test)]
mod tests {
    use crate::pieces::{Color, Kind, Piece, Position, ValidMove};

    #[test]
    fn bishop_valid_moves_initialy() {
        let bishop = Piece::new(Kind::Bishop, Color::Black, Position { x: 5, y: 4 });

        assert_eq!(bishop.valid_moves.len(), 13);
    }

    #[test]
    fn bishop_valid_moves_list_initialy() {
        let bishop = Piece::new(Kind::Bishop, Color::Black, Position { x: 5, y: 4 });

        assert_eq!(
            bishop.valid_moves,
            vec![
                ValidMove { x: 4, y: 5 }, // North-West
                ValidMove { x: 3, y: 6 },
                ValidMove { x: 2, y: 7 },
                ValidMove { x: 1, y: 8 },
                ValidMove { x: 6, y: 5 }, // North-East
                ValidMove { x: 7, y: 6 },
                ValidMove { x: 8, y: 7 },
                ValidMove { x: 4, y: 3 }, // South-West
                ValidMove { x: 3, y: 2 },
                ValidMove { x: 2, y: 1 },
                ValidMove { x: 6, y: 3 }, // South-East
                ValidMove { x: 7, y: 2 },
                ValidMove { x: 8, y: 1 },
            ]
        );
    }

    #[test]
    fn bishop_valid_moves_after_move() {
        let mut bishop = Piece::new(Kind::Bishop, Color::Black, Position { x: 4, y: 5 });

        bishop.make_a_move(5, 4);

        assert_eq!(bishop.valid_moves.len(), 13);
    }

    #[test]
    fn bishop_valid_moves_list_after_move() {
        let mut bishop = Piece::new(Kind::Bishop, Color::Black, Position { x: 4, y: 5 });

        bishop.make_a_move(5, 4);

        assert_eq!(
            bishop.valid_moves,
            vec![
                ValidMove { x: 4, y: 5 }, // North-West
                ValidMove { x: 3, y: 6 },
                ValidMove { x: 2, y: 7 },
                ValidMove { x: 1, y: 8 },
                ValidMove { x: 6, y: 5 }, // North-East
                ValidMove { x: 7, y: 6 },
                ValidMove { x: 8, y: 7 },
                ValidMove { x: 4, y: 3 }, // South-West
                ValidMove { x: 3, y: 2 },
                ValidMove { x: 2, y: 1 },
                ValidMove { x: 6, y: 3 }, // South-East
                ValidMove { x: 7, y: 2 },
                ValidMove { x: 8, y: 1 },
            ]
        );
    }

    #[test]
    fn bishop_valid_moves_initialy_can_only_move_south_east_when_placed_top_left_corner() {
        let bishop = Piece::new(Kind::Bishop, Color::Black, Position { x: 1, y: 8 });

        assert_eq!(
            bishop.valid_moves,
            vec![
                ValidMove { x: 2, y: 7 }, // South-East
                ValidMove { x: 3, y: 6 },
                ValidMove { x: 4, y: 5 },
                ValidMove { x: 5, y: 4 },
                ValidMove { x: 6, y: 3 },
                ValidMove { x: 7, y: 2 },
                ValidMove { x: 8, y: 1 },
            ]
        );
    }

    #[test]
    fn bishop_valid_moves_initialy_can_only_move_south_west_when_placed_top_right_corner() {
        let bishop = Piece::new(Kind::Bishop, Color::Black, Position { x: 8, y: 8 });

        assert_eq!(
            bishop.valid_moves,
            vec![
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
    fn bishop_valid_moves_initialy_can_only_move_north_east_when_placed_bottom_left_corner() {
        let bishop = Piece::new(Kind::Bishop, Color::Black, Position { x: 1, y: 1 });

        assert_eq!(
            bishop.valid_moves,
            vec![
                ValidMove { x: 2, y: 2 }, // North-East
                ValidMove { x: 3, y: 3 },
                ValidMove { x: 4, y: 4 },
                ValidMove { x: 5, y: 5 },
                ValidMove { x: 6, y: 6 },
                ValidMove { x: 7, y: 7 },
                ValidMove { x: 8, y: 8 },
            ]
        );
    }

    #[test]
    fn bishop_valid_moves_initialy_can_only_move_north_west_when_placed_bottom_right_corner() {
        let bishop = Piece::new(Kind::Bishop, Color::Black, Position { x: 8, y: 1 });

        assert_eq!(
            bishop.valid_moves,
            vec![
                ValidMove { x: 7, y: 2 }, // North-West
                ValidMove { x: 6, y: 3 },
                ValidMove { x: 5, y: 4 },
                ValidMove { x: 4, y: 5 },
                ValidMove { x: 3, y: 6 },
                ValidMove { x: 2, y: 7 },
                ValidMove { x: 1, y: 8 },
            ]
        );
    }
}
