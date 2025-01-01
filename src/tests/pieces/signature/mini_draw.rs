#[cfg(test)]
mod tests {
    use crate::pieces::{Color, Kind, Piece, Position};

    #[test]
    fn bishop_mini_drawing() {
        let bishop = Piece::new(Kind::Bishop, Color::White, Position { x: 1, y: 2 });

        assert_eq!(bishop.mini_draw(), "♗");
    }

    #[test]
    fn king_mini_drawing() {
        let king = Piece::new(Kind::King, Color::White, Position { x: 1, y: 2 });

        assert_eq!(king.mini_draw(), "♔");
    }

    #[test]
    fn knight_mini_drawing() {
        let knight = Piece::new(Kind::Knight, Color::White, Position { x: 1, y: 2 });

        assert_eq!(knight.mini_draw(), "♘");
    }

    #[test]
    fn pawn_mini_drawing() {
        let pawn = Piece::new(Kind::Pawn, Color::White, Position { x: 1, y: 2 });

        assert_eq!(pawn.mini_draw(), "♙");
    }

    #[test]
    fn queen_mini_drawing() {
        let queen = Piece::new(Kind::Queen, Color::White, Position { x: 1, y: 2 });

        assert_eq!(queen.mini_draw(), "♕");
    }

    #[test]
    fn rook_mini_drawing() {
        let rook = Piece::new(Kind::Rook, Color::White, Position { x: 1, y: 2 });

        assert_eq!(rook.mini_draw(), "♖");
    }
}
