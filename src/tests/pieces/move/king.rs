#[cfg(test)]
mod tests {
    use crate::pieces::{Color, Kind, Piece, Position, ValidMove};

    #[test]
    fn king_valid_moves_after_normal_move() {
        let mut king = Piece::new(Kind::King, Color::Black, Position { x: 2, y: 2 });

        king.make_a_move(3, 2);

        assert_eq!(king.valid_moves.len(), 8);
    }

    #[test]
    fn king_valid_moves_list_after_normal_move() {
        let mut pawn = Piece::new(Kind::King, Color::Black, Position { x: 2, y: 2 });

        pawn.make_a_move(3, 2);

        assert_eq!(
            pawn.valid_moves,
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
}
