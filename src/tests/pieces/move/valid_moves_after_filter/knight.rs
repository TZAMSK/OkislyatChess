#[cfg(test)]
mod tests {
    use crate::pieces::{Color, Kind, Piece, Position, ValidMove};

    #[test]
    fn knight_valid_moves_initialy() {
        let knight = Piece::new(Kind::Knight, Color::Black, Position { x: 4, y: 5 });

        assert_eq!(knight.valid_moves.len(), 8);
    }

    #[test]
    fn knight_valid_moves_list_initialy() {
        let knight = Piece::new(Kind::Knight, Color::Black, Position { x: 4, y: 5 });

        assert_eq!(
            knight.valid_moves,
            vec![
                ValidMove { x: 2, y: 6 }, // Top-Left
                ValidMove { x: 3, y: 7 },
                ValidMove { x: 5, y: 7 }, // Top-Right
                ValidMove { x: 6, y: 6 },
                ValidMove { x: 2, y: 4 }, // Bottom-Left
                ValidMove { x: 3, y: 3 },
                ValidMove { x: 5, y: 3 }, // Bottom-Right
                ValidMove { x: 6, y: 4 }
            ]
        );
    }

    #[test]
    fn knight_valid_moves_after_move() {
        let mut knight = Piece::new(Kind::Knight, Color::Black, Position { x: 3, y: 3 });

        knight.make_a_move(4, 5);

        assert_eq!(knight.valid_moves.len(), 8);
    }

    #[test]
    fn knight_valid_moves_list_after_move() {
        let mut knight = Piece::new(Kind::Knight, Color::Black, Position { x: 3, y: 3 });

        knight.make_a_move(4, 5);

        assert_eq!(
            knight.valid_moves,
            vec![
                ValidMove { x: 2, y: 6 }, // Top-Left
                ValidMove { x: 3, y: 7 },
                ValidMove { x: 5, y: 7 }, // Top-Right
                ValidMove { x: 6, y: 6 },
                ValidMove { x: 2, y: 4 }, // Bottom-Left
                ValidMove { x: 3, y: 3 },
                ValidMove { x: 5, y: 3 }, // Bottom-Right
                ValidMove { x: 6, y: 4 }
            ]
        );
    }

    #[test]
    fn knight_valid_moves_initialy_can_only_move_bottom_right_when_placed_top_left_corner() {
        let king = Piece::new(Kind::Knight, Color::Black, Position { x: 1, y: 8 });

        assert_eq!(
            king.valid_moves,
            vec![
                ValidMove { x: 2, y: 6 }, // Bottom-Right
                ValidMove { x: 3, y: 7 },
            ]
        );
    }

    #[test]
    fn knight_valid_moves_initialy_can_only_move_bottom_left_when_placed_top_right_corner() {
        let king = Piece::new(Kind::Knight, Color::Black, Position { x: 8, y: 8 });

        assert_eq!(
            king.valid_moves,
            vec![
                ValidMove { x: 6, y: 7 }, // Bottom-Left
                ValidMove { x: 7, y: 6 },
            ]
        );
    }

    #[test]
    fn knight_valid_moves_initialy_can_only_move_top_right_when_placed_bottom_left_corner() {
        let king = Piece::new(Kind::Knight, Color::Black, Position { x: 1, y: 1 });

        assert_eq!(
            king.valid_moves,
            vec![
                ValidMove { x: 2, y: 3 }, // Top-Right
                ValidMove { x: 3, y: 2 },
            ]
        );
    }

    #[test]
    fn knight_valid_moves_initialy_can_only_move_top_left_when_placed_bottom_right_corner() {
        let king = Piece::new(Kind::Knight, Color::Black, Position { x: 8, y: 1 });

        assert_eq!(
            king.valid_moves,
            vec![
                ValidMove { x: 6, y: 2 }, // Top-Left
                ValidMove { x: 7, y: 3 },
            ]
        );
    }
}
