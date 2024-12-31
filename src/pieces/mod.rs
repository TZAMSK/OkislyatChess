mod king;
mod pawn;

pub struct Piece {
    pub kind: Kind,
    pub color: Color,
    pub position: Position,
    pub valid_moves: Vec<ValidMove>,
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

#[derive(Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

pub type ValidMove = cgmath::Point2<i8>;

impl Piece {
    pub fn new(kind: Kind, color: Color, position: Position) -> Self {
        let mut piece = Piece {
            kind,
            color,
            position,
            valid_moves: Vec::new(),
        };

        match piece.kind {
            Kind::King => piece.update_king_moves(),
            Kind::Pawn => piece.update_pawn_moves(),
            _ => piece.update_pawn_moves(),
        }

        piece
    }

    pub fn make_a_move(&mut self, new_x: u8, new_y: u8) -> Position {
        let old_position = self.position.clone();
        self.position = Position { x: new_x, y: new_y };

        match self.kind {
            Kind::King => self.update_king_moves(),
            Kind::Pawn => self.update_pawn_moves(),
            _ => self.valid_moves.clear(),
        }

        old_position
    }
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

    fn sliding_moves(&self, directions: &[(i8, i8)]) -> Vec<ValidMove> {
        let mut moves = Vec::new();
        for &(x, y) in directions {
            for step in 1..8 {
                moves.push(ValidMove::new(x * step, y * step));
            }
        }

        moves
    }
}
