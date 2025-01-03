use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::app::{App, AppResult};
use crate::ui::constants::MENU_ITEMS;

pub fn key_events(app: &mut App) -> AppResult<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => return Err("err".to_string().into()),
                KeyCode::Up => app.menu_cursor_up(MENU_ITEMS.len() as u8),
                KeyCode::Down => app.menu_cursor_down(MENU_ITEMS.len() as u8),
                KeyCode::Enter => app.menu_item_selected(),
                _ => {}
            }
        }
    }

    Ok(())
}
