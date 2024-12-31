use super::{Kind, Piece, ValidMove};

impl Piece {
    pub fn update_king_moves(&mut self) {
        self.valid_moves.clear();

        let current_point = ValidMove {
            x: self.position.x as i8,
            y: self.position.y as i8,
        };

        for point in Kind::possible_moves(&self.kind).into_iter() {
            let new_x = current_point.x + point.x;
            let new_y = current_point.y + point.y;

            self.valid_moves.push(ValidMove { x: new_x, y: new_y })
        }
    }
}
