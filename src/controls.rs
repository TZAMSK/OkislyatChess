use crossterm::event::MouseEventKind;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::app::{App, AppResult};
use crate::ui::constants::MENU_ITEMS;

pub fn mouse_events(app: &mut App) -> AppResult<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Mouse(mouse_event) = event::read()? {
            if let MouseEventKind::Down(_) = mouse_event.kind {
                let mouse_x = mouse_event.column as u8;
                let mouse_y = mouse_event.row as u8;

                if let Some((row, col)) = app
                    .area
                    .mouse_to_piece_coords(mouse_x.into(), mouse_y.into())
                {
                    println!("Mouse clicked on cell: ({}, {})", row, col);
                }
            }
        }
    }
    Ok(())
}

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
