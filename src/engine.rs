pub struct Engine {
    pub menu_cursor: u8,
}

impl Default for Engine {
    fn default() -> Self {
        Self { menu_cursor: 0 }
    }
}

impl Engine {
    pub fn menu_cursor_up(&mut self, length: u8) {
        if self.menu_cursor > 0 {
            self.menu_cursor -= 1;
        } else {
            self.menu_cursor = length - 1;
        }
    }

    pub fn menu_cursor_down(&mut self, length: u8) {
        if self.menu_cursor < length - 1 {
            self.menu_cursor += 1;
        } else {
            self.menu_cursor = 0;
        }
    }
}