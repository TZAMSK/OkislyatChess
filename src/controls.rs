use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::engine::{AppResult, Engine};
use crate::ui::constants::MENU_ITEMS;

pub fn key_events(engine: &mut Engine) -> AppResult<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => return Err("err".to_string().into()),
                KeyCode::Up => engine.menu_cursor_up(MENU_ITEMS.len() as u8),
                KeyCode::Down => engine.menu_cursor_down(MENU_ITEMS.len() as u8),
                _ => {}
            }
        }
    }

    Ok(())
}
