use ratatui::layout::Rect;

pub struct Area {
    board_area: Rect,
}

impl Area {
    pub fn new() -> Area {
        Area {
            board_area: Rect::new(0, 0, 0, 0),
        }
    }

    pub fn board_area(&self) -> Rect {
        self.board_area
    }

    pub fn mouse_to_piece_coords(&self, mouse_x: u16, mouse_y: u16) -> Option<(u16, u16)> {
        let board_x = self.board_area.x;
        let board_y = self.board_area.y;
        let cell_width = self.board_area.width / 8;
        let cell_height = self.board_area.height / 8;

        if mouse_x >= board_x
            && mouse_x < board_x + self.board_area.width
            && mouse_y >= board_y
            && mouse_y < board_y + self.board_area.height
        {
            let col = (mouse_x - board_x) / cell_width;
            let row = (mouse_y - board_y) / cell_height;

            Some((row, col))
        } else {
            None
        }
    }
}
