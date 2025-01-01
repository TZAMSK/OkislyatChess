mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

use {
    bishop::Bishop, color_eyre::owo_colors::OwoColorize, king::King, knight::Knight, pawn::Pawn,
    queen::Queen, rook::Rook,
};

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
            Kind::Pawn => piece.update_pawn_moves(),
            _ => piece.updated_new_moves(),
        }

        piece
    }

    pub fn make_a_move(&mut self, new_x: u8, new_y: u8) -> Position {
        let old_position = self.position.clone();
        self.position = Position { x: new_x, y: new_y };

        match self.kind {
            Kind::Pawn => self.update_pawn_moves(),
            _ => self.updated_new_moves(),
        }

        old_position
    }

    fn updated_new_moves(&mut self) {
        let new_moves = self.new_moves();

        self.valid_moves = self.filtered_moves(new_moves)
    }

    fn filtered_moves(&self, new_moves: Vec<ValidMove>) -> Vec<ValidMove> {
        new_moves
            .into_iter()
            .filter(|move_| move_.x >= 1 && move_.x <= 8 && move_.y >= 1 && move_.y <= 8)
            .collect()
    }

    pub fn new_moves(&mut self) -> Vec<ValidMove> {
        let mut valid_moves = Vec::new();

        let current_point = ValidMove {
            x: self.position.x as i8,
            y: self.position.y as i8,
        };

        for point in Kind::possible_moves(&self.kind).into_iter() {
            let new_x = current_point.x + point.x;
            let new_y = current_point.y + point.y;

            valid_moves.push(ValidMove { x: new_x, y: new_y })
        }

        valid_moves
    }

    pub fn draw(&self) -> &str {
        match self.kind {
            Kind::Bishop => Rook::draw(),
            Kind::King => King::draw(),
            Kind::Knight => King::draw(),
            Kind::Pawn => Pawn::draw(),
            Kind::Queen => Queen::draw(),
            Kind::Rook => Rook::draw(),
        }
    }

    pub fn notation(&self) -> &str {
        match self.kind {
            Kind::Bishop => Rook::notation(),
            Kind::King => King::notation(),
            Kind::Knight => King::notation(),
            Kind::Pawn => Pawn::notation(),
            Kind::Queen => Queen::notation(),
            Kind::Rook => Rook::notation(),
        }
    }

    pub fn mini_draw(&self) -> &str {
        match self.kind {
            Kind::Bishop => Rook::mini_draw(),
            Kind::King => King::mini_draw(),
            Kind::Knight => King::mini_draw(),
            Kind::Pawn => Pawn::mini_draw(),
            Kind::Queen => Queen::mini_draw(),
            Kind::Rook => Rook::mini_draw(),
        }
    }
}

impl Kind {
    pub fn possible_moves(&self) -> Vec<ValidMove> {
        match self {
            Kind::Bishop => self.sliding_moves(&[
                (-1, 1),  // North-East
                (1, 1),   // North-West
                (-1, -1), // South-East
                (1, -1),  // South-West
            ]),
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
                (-1, 0),  // West
                (-1, 1),  // North-West
                (0, 1),   // North
                (1, 1),   // North-East
                (1, 0),   // East
                (1, -1),  // South-East
                (0, -1),  // South
                (-1, -1), // South-West
            ]),
            Kind::Rook => self.sliding_moves(&[
                (-1, 0), // East
                (0, 1),  // North
                (1, 0),  // West
                (0, -1), // South
            ]),
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
