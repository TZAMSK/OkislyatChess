#[cfg(test)]
mod tests {
    use crate::pieces::{Color, Kind, Piece, Position, ValidMove};

    #[test]
    fn knight_valid_moves_initialy() {
        let mut knight = Piece::new(Kind::Knight, Color::Black, Position { x: 4, y: 5 });

        assert_eq!(knight.new_moves().len(), 8);
    }

    #[test]
    fn knight_valid_moves_list_initialy() {
        let mut knight = Piece::new(Kind::Knight, Color::Black, Position { x: 4, y: 5 });

        assert_eq!(
            knight.new_moves(),
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

        assert_eq!(knight.new_moves().len(), 8);
    }

    #[test]
    fn knight_valid_moves_list_after_move() {
        let mut knight = Piece::new(Kind::Knight, Color::Black, Position { x: 3, y: 3 });

        knight.make_a_move(4, 5);

        assert_eq!(
            knight.new_moves(),
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
}
