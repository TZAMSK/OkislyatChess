use std::error::Error;

use crate::area::Area;

pub struct App {
    pub menu_cursor: u8,
    pub current_screen: Screen,
    pub mouse_position: Option<(u8, u8)>,
    pub area: Area,
}

pub enum Screen {
    Menu,
    TwoPlayers,
    Bot,
    Help,
}

pub type AppResult<T> = std::result::Result<T, Box<dyn Error>>;

impl Default for App {
    fn default() -> Self {
        Self {
            menu_cursor: 0,
            current_screen: Screen::Menu,
            mouse_position: None,
            area: Area::new(),
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
            2 => self.current_screen = Screen::Help,
            _ => {}
        }
    }

    pub fn mouse_position(&self) -> Option<(u8, u8)> {
        self.mouse_position
    }

    pub fn handle_mouse_position(&mut self, x: u8, y: u8) {
        self.mouse_position = Some((x, y))
    }
}
