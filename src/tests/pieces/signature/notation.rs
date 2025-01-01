#[cfg(test)]
mod tests {
    use crate::pieces::{Color, Kind, Piece, Position};

    #[test]
    fn bishop_notation() {
        let bishop = Piece::new(Kind::Bishop, Color::White, Position { x: 1, y: 2 });

        assert_eq!(bishop.notation(), "B");
    }

    #[test]
    fn king_notation() {
        let king = Piece::new(Kind::King, Color::White, Position { x: 1, y: 2 });

        assert_eq!(king.notation(), "K");
    }

    #[test]
    fn knight_notation() {
        let knight = Piece::new(Kind::Knight, Color::White, Position { x: 1, y: 2 });

        assert_eq!(knight.notation(), "N");
    }

    #[test]
    fn pawn_notation() {
        let pawn = Piece::new(Kind::Pawn, Color::White, Position { x: 1, y: 2 });

        assert_eq!(pawn.notation(), "P");
    }

    #[test]
    fn queen_notation() {
        let queen = Piece::new(Kind::Queen, Color::White, Position { x: 1, y: 2 });

        assert_eq!(queen.notation(), "Q");
    }

    #[test]
    fn rook_notation() {
        let rook = Piece::new(Kind::Rook, Color::White, Position { x: 1, y: 2 });

        assert_eq!(rook.notation(), "R");
    }
}
