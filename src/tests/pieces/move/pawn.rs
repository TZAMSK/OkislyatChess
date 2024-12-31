#[cfg(test)]
mod tests {
    use crate::pieces::{Color, Kind, Move, Piece, Position, ValidMove};

    #[test]
    fn pawn_valid_moves_after_double_jump() {
        let mut pawn = Piece::new(Kind::Pawn, Color::Black, Position { x: 1, y: 2 });

        pawn.make_a_move(1, 4);

        assert_eq!(pawn.valid_moves.len(), 3);

        pawn.make_a_move(1, 5);

        assert_eq!(pawn.valid_moves.len(), 3);
    }

    #[test]
    fn pawn_valid_moves_list_after_double_jump() {
        let mut pawn = Piece::new(Kind::Pawn, Color::Black, Position { x: 2, y: 2 });

        pawn.make_a_move(2, 4);

        assert_eq!(
            pawn.valid_moves,
            vec![
                ValidMove { x: 1, y: 5 },
                ValidMove { x: 2, y: 5 },
                ValidMove { x: 3, y: 5 }
            ]
        );
    }

    #[test]
    fn pawn_valid_moves_after_normal_jump() {
        let mut pawn = Piece::new(Kind::Pawn, Color::Black, Position { x: 1, y: 2 });

        pawn.make_a_move(1, 3);

        assert_eq!(pawn.valid_moves.len(), 3);
    }

    #[test]
    fn pawn_valid_moves_list_after_normal_jump() {
        let mut pawn = Piece::new(Kind::Pawn, Color::Black, Position { x: 2, y: 2 });

        pawn.make_a_move(2, 3);

        assert_eq!(
            pawn.valid_moves,
            vec![
                ValidMove { x: 1, y: 4 },
                ValidMove { x: 2, y: 4 },
                ValidMove { x: 3, y: 4 }
            ]
        );
    }

    #[test]
    fn pawn_can_double_jump_initialy() {
        let pawn = Piece::new(Kind::Pawn, Color::Black, Position { x: 3, y: 2 });

        assert_eq!(pawn.valid_moves.len(), 4);
    }

    #[test]
    fn pawn_can_double_jump_initialy_list() {
        let pawn = Piece::new(Kind::Pawn, Color::Black, Position { x: 3, y: 2 });

        assert_eq!(
            pawn.valid_moves,
            vec![
                ValidMove { x: 3, y: 4 },
                ValidMove { x: 2, y: 3 },
                ValidMove { x: 3, y: 3 },
                ValidMove { x: 4, y: 3 }
            ]
        );
    }
}
