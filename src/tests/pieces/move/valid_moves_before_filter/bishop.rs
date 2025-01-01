#[cfg(test)]
mod tests {
    use crate::pieces::{Color, Kind, Piece, Position, ValidMove};

    #[test]
    fn bishop_valid_moves_initialy() {
        let mut bishop = Piece::new(Kind::Bishop, Color::Black, Position { x: 5, y: 4 });

        assert_eq!(bishop.new_moves().len(), 28);
    }

    #[test]
    fn bishop_valid_moves_list_initialy() {
        let mut bishop = Piece::new(Kind::Bishop, Color::Black, Position { x: 5, y: 4 });

        assert_eq!(
            bishop.new_moves(),
            vec![
                ValidMove { x: 4, y: 5 }, // North-West
                ValidMove { x: 3, y: 6 },
                ValidMove { x: 2, y: 7 },
                ValidMove { x: 1, y: 8 },
                ValidMove { x: 0, y: 9 },
                ValidMove { x: -1, y: 10 },
                ValidMove { x: -2, y: 11 },
                ValidMove { x: 6, y: 5 }, // North-East
                ValidMove { x: 7, y: 6 },
                ValidMove { x: 8, y: 7 },
                ValidMove { x: 9, y: 8 },
                ValidMove { x: 10, y: 9 },
                ValidMove { x: 11, y: 10 },
                ValidMove { x: 12, y: 11 },
                ValidMove { x: 4, y: 3 }, // South-West
                ValidMove { x: 3, y: 2 },
                ValidMove { x: 2, y: 1 },
                ValidMove { x: 1, y: 0 },
                ValidMove { x: 0, y: -1 },
                ValidMove { x: -1, y: -2 },
                ValidMove { x: -2, y: -3 },
                ValidMove { x: 6, y: 3 }, // South-Est
                ValidMove { x: 7, y: 2 },
                ValidMove { x: 8, y: 1 },
                ValidMove { x: 9, y: 0 },
                ValidMove { x: 10, y: -1 },
                ValidMove { x: 11, y: -2 },
                ValidMove { x: 12, y: -3 }
            ]
        );
    }

    #[test]
    fn bishop_valid_moves_after_move() {
        let mut bishop = Piece::new(Kind::Bishop, Color::Black, Position { x: 4, y: 5 });

        bishop.make_a_move(5, 4);

        assert_eq!(bishop.new_moves().len(), 28);
    }

    #[test]
    fn bishop_valid_moves_list_after_move() {
        let mut bishop = Piece::new(Kind::Bishop, Color::Black, Position { x: 4, y: 5 });

        bishop.make_a_move(5, 4);

        assert_eq!(
            bishop.new_moves(),
            vec![
                ValidMove { x: 4, y: 5 }, // North-West
                ValidMove { x: 3, y: 6 },
                ValidMove { x: 2, y: 7 },
                ValidMove { x: 1, y: 8 },
                ValidMove { x: 0, y: 9 },
                ValidMove { x: -1, y: 10 },
                ValidMove { x: -2, y: 11 },
                ValidMove { x: 6, y: 5 }, // North-East
                ValidMove { x: 7, y: 6 },
                ValidMove { x: 8, y: 7 },
                ValidMove { x: 9, y: 8 },
                ValidMove { x: 10, y: 9 },
                ValidMove { x: 11, y: 10 },
                ValidMove { x: 12, y: 11 },
                ValidMove { x: 4, y: 3 }, // South-West
                ValidMove { x: 3, y: 2 },
                ValidMove { x: 2, y: 1 },
                ValidMove { x: 1, y: 0 },
                ValidMove { x: 0, y: -1 },
                ValidMove { x: -1, y: -2 },
                ValidMove { x: -2, y: -3 },
                ValidMove { x: 6, y: 3 }, // South-East
                ValidMove { x: 7, y: 2 },
                ValidMove { x: 8, y: 1 },
                ValidMove { x: 9, y: 0 },
                ValidMove { x: 10, y: -1 },
                ValidMove { x: 11, y: -2 },
                ValidMove { x: 12, y: -3 }
            ]
        );
    }
}
