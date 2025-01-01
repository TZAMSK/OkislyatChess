#[cfg(test)]
mod tests {
    use crate::pieces::{Color, Kind, Piece, Position, ValidMove};

    #[test]
    fn queen_valid_moves_initialy() {
        let mut queen = Piece::new(Kind::Queen, Color::Black, Position { x: 5, y: 5 });

        assert_eq!(queen.new_moves().len(), 56);
    }

    #[test]
    fn queen_valid_moves_list_initialy() {
        let mut queen = Piece::new(Kind::Queen, Color::Black, Position { x: 5, y: 5 });

        assert_eq!(
            queen.new_moves(),
            vec![
                ValidMove { x: 4, y: 5 }, // Ouest
                ValidMove { x: 3, y: 5 },
                ValidMove { x: 2, y: 5 },
                ValidMove { x: 1, y: 5 },
                ValidMove { x: 0, y: 5 },
                ValidMove { x: -1, y: 5 },
                ValidMove { x: -2, y: 5 },
                ValidMove { x: 4, y: 6 }, // North-Ouest
                ValidMove { x: 3, y: 7 },
                ValidMove { x: 2, y: 8 },
                ValidMove { x: 1, y: 9 },
                ValidMove { x: 0, y: 10 },
                ValidMove { x: -1, y: 11 },
                ValidMove { x: -2, y: 12 },
                ValidMove { x: 5, y: 6 }, // North
                ValidMove { x: 5, y: 7 },
                ValidMove { x: 5, y: 8 },
                ValidMove { x: 5, y: 9 },
                ValidMove { x: 5, y: 10 },
                ValidMove { x: 5, y: 11 },
                ValidMove { x: 5, y: 12 },
                ValidMove { x: 6, y: 6 }, // North-Est
                ValidMove { x: 7, y: 7 },
                ValidMove { x: 8, y: 8 },
                ValidMove { x: 9, y: 9 },
                ValidMove { x: 10, y: 10 },
                ValidMove { x: 11, y: 11 },
                ValidMove { x: 12, y: 12 },
                ValidMove { x: 6, y: 5 }, // Est
                ValidMove { x: 7, y: 5 },
                ValidMove { x: 8, y: 5 },
                ValidMove { x: 9, y: 5 },
                ValidMove { x: 10, y: 5 },
                ValidMove { x: 11, y: 5 },
                ValidMove { x: 12, y: 5 },
                ValidMove { x: 6, y: 4 }, // South-Est
                ValidMove { x: 7, y: 3 },
                ValidMove { x: 8, y: 2 },
                ValidMove { x: 9, y: 1 },
                ValidMove { x: 10, y: 0 },
                ValidMove { x: 11, y: -1 },
                ValidMove { x: 12, y: -2 },
                ValidMove { x: 5, y: 4 }, // Sud
                ValidMove { x: 5, y: 3 },
                ValidMove { x: 5, y: 2 },
                ValidMove { x: 5, y: 1 },
                ValidMove { x: 5, y: 0 },
                ValidMove { x: 5, y: -1 },
                ValidMove { x: 5, y: -2 },
                ValidMove { x: 4, y: 4 }, // South-Ouest
                ValidMove { x: 3, y: 3 },
                ValidMove { x: 2, y: 2 },
                ValidMove { x: 1, y: 1 },
                ValidMove { x: 0, y: 0 },
                ValidMove { x: -1, y: -1 },
                ValidMove { x: -2, y: -2 }
            ]
        );
    }

    #[test]
    fn queen_valid_moves_after_move() {
        let mut queen = Piece::new(Kind::Queen, Color::Black, Position { x: 4, y: 5 });

        queen.make_a_move(5, 5);

        assert_eq!(queen.new_moves().len(), 56);
    }

    #[test]
    fn queen_valid_moves_list_after_move() {
        let mut queen = Piece::new(Kind::Queen, Color::Black, Position { x: 4, y: 5 });

        queen.make_a_move(5, 5);

        assert_eq!(
            queen.new_moves(),
            vec![
                ValidMove { x: 4, y: 5 }, // Ouest
                ValidMove { x: 3, y: 5 },
                ValidMove { x: 2, y: 5 },
                ValidMove { x: 1, y: 5 },
                ValidMove { x: 0, y: 5 },
                ValidMove { x: -1, y: 5 },
                ValidMove { x: -2, y: 5 },
                ValidMove { x: 4, y: 6 }, // North-Ouest
                ValidMove { x: 3, y: 7 },
                ValidMove { x: 2, y: 8 },
                ValidMove { x: 1, y: 9 },
                ValidMove { x: 0, y: 10 },
                ValidMove { x: -1, y: 11 },
                ValidMove { x: -2, y: 12 },
                ValidMove { x: 5, y: 6 }, // North
                ValidMove { x: 5, y: 7 },
                ValidMove { x: 5, y: 8 },
                ValidMove { x: 5, y: 9 },
                ValidMove { x: 5, y: 10 },
                ValidMove { x: 5, y: 11 },
                ValidMove { x: 5, y: 12 },
                ValidMove { x: 6, y: 6 }, // North-Est
                ValidMove { x: 7, y: 7 },
                ValidMove { x: 8, y: 8 },
                ValidMove { x: 9, y: 9 },
                ValidMove { x: 10, y: 10 },
                ValidMove { x: 11, y: 11 },
                ValidMove { x: 12, y: 12 },
                ValidMove { x: 6, y: 5 }, // Est
                ValidMove { x: 7, y: 5 },
                ValidMove { x: 8, y: 5 },
                ValidMove { x: 9, y: 5 },
                ValidMove { x: 10, y: 5 },
                ValidMove { x: 11, y: 5 },
                ValidMove { x: 12, y: 5 },
                ValidMove { x: 6, y: 4 }, // South-Est
                ValidMove { x: 7, y: 3 },
                ValidMove { x: 8, y: 2 },
                ValidMove { x: 9, y: 1 },
                ValidMove { x: 10, y: 0 },
                ValidMove { x: 11, y: -1 },
                ValidMove { x: 12, y: -2 },
                ValidMove { x: 5, y: 4 }, // Sud
                ValidMove { x: 5, y: 3 },
                ValidMove { x: 5, y: 2 },
                ValidMove { x: 5, y: 1 },
                ValidMove { x: 5, y: 0 },
                ValidMove { x: 5, y: -1 },
                ValidMove { x: 5, y: -2 },
                ValidMove { x: 4, y: 4 }, // South-Ouest
                ValidMove { x: 3, y: 3 },
                ValidMove { x: 2, y: 2 },
                ValidMove { x: 1, y: 1 },
                ValidMove { x: 0, y: 0 },
                ValidMove { x: -1, y: -1 },
                ValidMove { x: -2, y: -2 }
            ]
        );
    }
}
