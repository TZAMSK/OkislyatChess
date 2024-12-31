mod pawn;

pub struct Piece {
    kind: Kind,
    color: Color,
    position: Position,
    valid_moves: Vec<ValidMove>,
    //menaces: Vec<Piece>,
}

pub enum Kind {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

pub enum Color {
    Black,
    White,
}

pub struct Position {
    x: u32,
    y: u32,
}

pub type ValidMove = cgmath::Point2<i32>;

pub trait Move {
    fn make_a_move(&self, new_x: u32, new_y: u32) -> Position;
    fn valid_position(&self, new_x: u32, new_y: u32) -> Position;
}

impl Kind {
    pub fn possible_moves(&self) -> Vec<ValidMove> {
        match self {
            Kind::Bishop => self.sliding_moves(&[(-1, 1), (1, 1), (-1, -1), (1, -1)]),
            Kind::King => vec![
                ValidMove { x: -1, y: 1 }, // Around
                ValidMove { x: 0, y: 1 },
                ValidMove { x: 1, y: 1 },
                ValidMove { x: -1, y: 0 },
                ValidMove { x: 1, y: 0 },
                ValidMove { x: -1, y: -1 },
                ValidMove { x: 0, y: -1 },
                ValidMove { x: 1, y: -1 },
            ],
            Kind::Knight => vec![
                ValidMove { x: -2, y: 1 }, // Top-Left
                ValidMove { x: -1, y: 2 },
                ValidMove { x: 1, y: 2 }, // Top-Right
                ValidMove { x: 2, y: 1 },
                ValidMove { x: -2, y: -1 }, // Bottom-Left
                ValidMove { x: -1, y: -2 },
                ValidMove { x: 1, y: -2 }, // Bottom-Right
                ValidMove { x: 2, y: -1 },
            ],
            Kind::Pawn => vec![
                ValidMove { x: -1, y: 1 },
                ValidMove { x: 0, y: 1 },
                ValidMove { x: 1, y: 1 },
                ValidMove { x: 0, y: 2 },
            ],
            Kind::Queen => self.sliding_moves(&[
                (-1, 0),  // Ouest
                (-1, 1),  // North-Ouest
                (0, 1),   // North
                (1, 1),   // North-Est
                (1, 0),   // Est
                (1, -1),  // South-Est
                (0, -1),  // South
                (-1, -1), // South-Ouest
            ]),
            Kind::Rook => self.sliding_moves(&[(-1, 0), (0, 1), (1, 0), (0, -1)]),
        }
    }

    fn sliding_moves(&self, directions: &[(i32, i32)]) -> Vec<ValidMove> {
        let mut moves = Vec::new();
        for &(x, y) in directions {
            for step in 1..5 {
                moves.push(ValidMove::new(x * step, y * step));
            }
        }

        moves
    }
}
