use super::{Kind, Move, Piece, Position, ValidMove};

impl Move for Piece {
    fn make_a_move(&mut self, new_x: u8, new_y: u8) -> Position {
        let old_position = self.position.clone();
        self.position = Position { x: new_x, y: new_y };

        match self.kind {
            Kind::Pawn => {
                self.update_pawn_moves();
            }
            _ => self.valid_moves.clear(),
        }

        old_position
    }
}

impl Piece {
    pub fn update_pawn_moves(&mut self) {
        self.valid_moves.clear();

        if self.position.y == 2 {
            self.valid_moves.push(ValidMove {
                x: self.position.x as i8,
                y: (self.position.y + 2) as i8,
            });
        }
        self.valid_moves.push(ValidMove {
            x: (self.position.x - 1) as i8,
            y: (self.position.y + 1) as i8,
        });

        self.valid_moves.push(ValidMove {
            x: self.position.x as i8,
            y: (self.position.y + 1) as i8,
        });

        self.valid_moves.push(ValidMove {
            x: (self.position.x + 1) as i8,
            y: (self.position.y + 1) as i8,
        });
    }
}
