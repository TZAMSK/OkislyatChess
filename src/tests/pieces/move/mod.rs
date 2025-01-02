mod valid_moves_after_filter;
mod valid_moves_before_filter;

use crate::pieces::{Kind, Piece, Position};

impl Piece {
    pub fn make_a_move(&mut self, new_x: u8, new_y: u8) -> Position {
        let old_position = self.position.clone();
        self.position = Position { x: new_x, y: new_y };

        match self.kind {
            Kind::Pawn => self.update_pawn_moves(),
            _ => self.updated_new_moves(),
        }

        old_position
    }
}
