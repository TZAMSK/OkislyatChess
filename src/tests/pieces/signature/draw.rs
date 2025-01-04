#[cfg(test)]
mod tests {
    use crate::pieces::{Color, Kind, Piece, Position};

    #[test]
    fn bishop_drawing() {
        let bishop = Piece::new(Kind::Bishop, Color::White, Position { x: 1, y: 2 });

        assert_eq!(
            bishop.draw(),
            "\
    \n\
       ⭘\n\
    ▗██✝██▖\n\
    ▗█████▖\n\
     █████\n\
      ███\n\
   ▗███████▖\n\
    "
        );
    }

    #[test]
    fn king_drawing() {
        let king = Piece::new(Kind::King, Color::White, Position { x: 1, y: 2 });

        assert_eq!(
            king.draw(),
            "\
            \n\
      ✚\n\
    ▞▀▄▀▚\n\
   ▐▙▄█▄▟▌\n\
    ▐███▌\n\
     ███\n\
  ▗███████▖\n\
    "
        );
    }

    #[test]
    fn knight_drawing() {
        let knight = Piece::new(Kind::Knight, Color::White, Position { x: 1, y: 2 });

        assert_eq!(
            knight.draw(),
            "\
            \n\
    \n\
    ▟█▛██▙\n\
   ▟██████\n\
   ▀▀▀▐██▌\n\
    ▟███▌\n\
   ▟██████\n\
    "
        );
    }

    #[test]
    fn pawn_drawing() {
        let pawn = Piece::new(Kind::Pawn, Color::White, Position { x: 1, y: 2 });

        assert_eq!(
            pawn.draw(),
            "\
            \n\
        \n\
      ▟█▙\n\
     █████\n\
     █████\n\
      ███\n\
    ▟█████▙\n\
    "
        );
    }

    #[test]
    fn queen_drawing() {
        let queen = Piece::new(Kind::Queen, Color::White, Position { x: 1, y: 2 });

        assert_eq!(
            queen.draw(),
            "\
            \n\
◀█▟███▙█▶\n\
  ◥█◈█◤\n\
 ▗█████▖\n\
  █████\n\
   ███\n\
▗███████▖\n\
    "
        );
    }

    #[test]
    fn rook_drawing() {
        let rook = Piece::new(Kind::Rook, Color::White, Position { x: 1, y: 2 });

        assert_eq!(
            rook.draw(),
            "\
            \n\
    \n\
   ██▟█▙██\n\
   ▜█████▛\n\
    ▐███▌\n\
    ▐███▌\n\
  ▗███████▖\n\
    "
        );
    }
}
