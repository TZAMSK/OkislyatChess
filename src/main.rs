mod board;
mod engine;
mod pieces;
mod tests;
mod ui;

use crate::ui::constants::MENU_ITEMS;
use crate::ui::menu::render;
use engine::Engine;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::crossterm::event::KeyEventKind;
use ratatui::DefaultTerminal;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut engine = Engine::default();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            render::render_menu_ui(f, &engine, size);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Up => engine.menu_cursor_up(MENU_ITEMS.len() as u8),
                    KeyCode::Down => engine.menu_cursor_down(MENU_ITEMS.len() as u8),
                    _ => {}
                }
            }
        }
    }
}
