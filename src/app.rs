use std::error::Error;

pub struct App {
    pub menu_cursor: u8,
    pub current_screen: Screen,
}

enum Screen {
    Menu,
    TwoPlayers,
    Bot,
}

pub type AppResult<T> = std::result::Result<T, Box<dyn Error>>;

impl Default for App {
    fn default() -> Self {
        Self {
            menu_cursor: 0,
            current_screen: Screen::Menu,
        }
    }
}

impl App {
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

    pub fn back_to_menu(&mut self) {
        self.current_screen = Screen::Menu;
    }

    pub fn menu_item_selected(&mut self) {
        match self.menu_cursor {
            0 => self.current_screen = Screen::TwoPlayers,
            1 => self.current_screen = Screen::Bot,
            _ => {}
        }
    }
}
