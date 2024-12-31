use super::{Kind, Move, Piece, Position};

impl Move for Piece {
    fn make_a_move(&self, new_x: u32, new_y: u32) -> Position {
        Position {
            x: self.position.x,
            y: self.position.y,
        }
    }

    fn valid_position(&self, new_x: u32, new_y: u32) -> Position {
        Position {
            x: self.position.x,
            y: self.position.y,
        }
    }
}
