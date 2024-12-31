#[cfg(test)]
mod tests {
    use crate::pieces::{Color, Kind, Piece, Position, ValidMove};

    #[test]
    fn king_valid_moves_initialy() {
        let king = Piece::new(Kind::Knight, Color::Black, Position { x: 4, y: 5 });

        assert_eq!(king.valid_moves.len(), 8);
    }

    #[test]
    fn king_valid_moves_list_initialy() {
        let pawn = Piece::new(Kind::Knight, Color::Black, Position { x: 4, y: 5 });

        assert_eq!(
            pawn.valid_moves,
            vec![
                ValidMove { x: 2, y: 6 },
                ValidMove { x: 3, y: 7 },
                ValidMove { x: 5, y: 7 },
                ValidMove { x: 6, y: 6 },
                ValidMove { x: 2, y: 4 },
                ValidMove { x: 3, y: 3 },
                ValidMove { x: 5, y: 3 },
                ValidMove { x: 6, y: 4 }
            ]
        );
    }

    #[test]
    fn king_valid_moves_after_move() {
        let mut king = Piece::new(Kind::Knight, Color::Black, Position { x: 3, y: 3 });

        king.make_a_move(4, 5);

        assert_eq!(king.valid_moves.len(), 8);
    }

    #[test]
    fn king_valid_moves_list_after_move() {
        let mut pawn = Piece::new(Kind::Knight, Color::Black, Position { x: 3, y: 3 });

        pawn.make_a_move(4, 5);

        assert_eq!(
            pawn.valid_moves,
            vec![
                ValidMove { x: 2, y: 6 },
                ValidMove { x: 3, y: 7 },
                ValidMove { x: 5, y: 7 },
                ValidMove { x: 6, y: 6 },
                ValidMove { x: 2, y: 4 },
                ValidMove { x: 3, y: 3 },
                ValidMove { x: 5, y: 3 },
                ValidMove { x: 6, y: 4 }
            ]
        );
    }
}
