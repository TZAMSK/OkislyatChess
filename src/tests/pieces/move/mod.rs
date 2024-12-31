mod king;
mod pawn;

#[cfg(test)]
mod tests {
    use crate::pieces::{Kind, ValidMove};

    #[test]
    fn list_of_moves_bishop() {
        assert_eq!(
            Kind::Bishop.possible_moves(),
            vec![
                ValidMove { x: -1, y: 1 }, // North-Ouest
                ValidMove { x: -2, y: 2 },
                ValidMove { x: -3, y: 3 },
                ValidMove { x: -4, y: 4 },
                ValidMove { x: -5, y: 5 },
                ValidMove { x: -6, y: 6 },
                ValidMove { x: -7, y: 7 },
                ValidMove { x: 1, y: 1 }, // North-Est
                ValidMove { x: 2, y: 2 },
                ValidMove { x: 3, y: 3 },
                ValidMove { x: 4, y: 4 },
                ValidMove { x: 5, y: 5 },
                ValidMove { x: 6, y: 6 },
                ValidMove { x: 7, y: 7 },
                ValidMove { x: -1, y: -1 }, // South-Ouest
                ValidMove { x: -2, y: -2 },
                ValidMove { x: -3, y: -3 },
                ValidMove { x: -4, y: -4 },
                ValidMove { x: -5, y: -5 },
                ValidMove { x: -6, y: -6 },
                ValidMove { x: -7, y: -7 },
                ValidMove { x: 1, y: -1 }, // South-Est
                ValidMove { x: 2, y: -2 },
                ValidMove { x: 3, y: -3 },
                ValidMove { x: 4, y: -4 },
                ValidMove { x: 5, y: -5 },
                ValidMove { x: 6, y: -6 },
                ValidMove { x: 7, y: -7 },
            ]
        )
    }

    #[test]
    fn list_of_moves_king() {
        assert_eq!(
            Kind::King.possible_moves(),
            vec![
                ValidMove { x: -1, y: 1 }, // Around
                ValidMove { x: 0, y: 1 },
                ValidMove { x: 1, y: 1 },
                ValidMove { x: -1, y: 0 },
                ValidMove { x: 1, y: 0 },
                ValidMove { x: -1, y: -1 },
                ValidMove { x: 0, y: -1 },
                ValidMove { x: 1, y: -1 },
            ]
        )
    }

    #[test]
    fn list_of_moves_knight() {
        assert_eq!(
            Kind::Knight.possible_moves(),
            vec![
                ValidMove { x: -2, y: 1 }, // Top-Left
                ValidMove { x: -1, y: 2 },
                ValidMove { x: 1, y: 2 }, // Top-Right
                ValidMove { x: 2, y: 1 },
                ValidMove { x: -2, y: -1 }, // Bottom-Left
                ValidMove { x: -1, y: -2 },
                ValidMove { x: 1, y: -2 }, // Bottom-Right
                ValidMove { x: 2, y: -1 },
            ]
        )
    }

    #[test]
    fn list_of_moves_pawn() {
        assert_eq!(
            Kind::Pawn.possible_moves(),
            vec![
                ValidMove { x: -1, y: 1 },
                ValidMove { x: 0, y: 1 },
                ValidMove { x: 1, y: 1 },
                ValidMove { x: 0, y: 2 }
            ]
        )
    }

    #[test]
    fn list_of_moves_queen() {
        assert_eq!(
            Kind::Queen.possible_moves(),
            vec![
                ValidMove { x: -1, y: 0 }, // Ouest
                ValidMove { x: -2, y: 0 },
                ValidMove { x: -3, y: 0 },
                ValidMove { x: -4, y: 0 },
                ValidMove { x: -5, y: 0 },
                ValidMove { x: -6, y: 0 },
                ValidMove { x: -7, y: 0 },
                ValidMove { x: -1, y: 1 }, // North-Ouest
                ValidMove { x: -2, y: 2 },
                ValidMove { x: -3, y: 3 },
                ValidMove { x: -4, y: 4 },
                ValidMove { x: -5, y: 5 },
                ValidMove { x: -6, y: 6 },
                ValidMove { x: -7, y: 7 },
                ValidMove { x: 0, y: 1 }, // North
                ValidMove { x: 0, y: 2 },
                ValidMove { x: 0, y: 3 },
                ValidMove { x: 0, y: 4 },
                ValidMove { x: 0, y: 5 },
                ValidMove { x: 0, y: 6 },
                ValidMove { x: 0, y: 7 },
                ValidMove { x: 1, y: 1 }, // North-Est
                ValidMove { x: 2, y: 2 },
                ValidMove { x: 3, y: 3 },
                ValidMove { x: 4, y: 4 },
                ValidMove { x: 5, y: 5 },
                ValidMove { x: 6, y: 6 },
                ValidMove { x: 7, y: 7 },
                ValidMove { x: 1, y: 0 }, // Est
                ValidMove { x: 2, y: 0 },
                ValidMove { x: 3, y: 0 },
                ValidMove { x: 4, y: 0 },
                ValidMove { x: 5, y: 0 },
                ValidMove { x: 6, y: 0 },
                ValidMove { x: 7, y: 0 },
                ValidMove { x: 1, y: -1 }, // South-Est
                ValidMove { x: 2, y: -2 },
                ValidMove { x: 3, y: -3 },
                ValidMove { x: 4, y: -4 },
                ValidMove { x: 5, y: -5 },
                ValidMove { x: 6, y: -6 },
                ValidMove { x: 7, y: -7 },
                ValidMove { x: 0, y: -1 }, // South
                ValidMove { x: 0, y: -2 },
                ValidMove { x: 0, y: -3 },
                ValidMove { x: 0, y: -4 },
                ValidMove { x: 0, y: -5 },
                ValidMove { x: 0, y: -6 },
                ValidMove { x: 0, y: -7 },
                ValidMove { x: -1, y: -1 }, // South-Ouest
                ValidMove { x: -2, y: -2 },
                ValidMove { x: -3, y: -3 },
                ValidMove { x: -4, y: -4 },
                ValidMove { x: -5, y: -5 },
                ValidMove { x: -6, y: -6 },
                ValidMove { x: -7, y: -7 },
            ]
        )
    }

    #[test]
    fn list_of_moves_rook() {
        assert_eq!(
            Kind::Rook.possible_moves(),
            vec![
                ValidMove { x: -1, y: 0 }, // Ouest
                ValidMove { x: -2, y: 0 },
                ValidMove { x: -3, y: 0 },
                ValidMove { x: -4, y: 0 },
                ValidMove { x: -5, y: 0 },
                ValidMove { x: -6, y: 0 },
                ValidMove { x: -7, y: 0 },
                ValidMove { x: 0, y: 1 }, // North
                ValidMove { x: 0, y: 2 },
                ValidMove { x: 0, y: 3 },
                ValidMove { x: 0, y: 4 },
                ValidMove { x: 0, y: 5 },
                ValidMove { x: 0, y: 6 },
                ValidMove { x: 0, y: 7 },
                ValidMove { x: 1, y: 0 }, // Est
                ValidMove { x: 2, y: 0 },
                ValidMove { x: 3, y: 0 },
                ValidMove { x: 4, y: 0 },
                ValidMove { x: 5, y: 0 },
                ValidMove { x: 6, y: 0 },
                ValidMove { x: 7, y: 0 },
                ValidMove { x: 0, y: -1 }, // South
                ValidMove { x: 0, y: -2 },
                ValidMove { x: 0, y: -3 },
                ValidMove { x: 0, y: -4 },
                ValidMove { x: 0, y: -5 },
                ValidMove { x: 0, y: -6 },
                ValidMove { x: 0, y: -7 },
            ]
        )
    }
}
