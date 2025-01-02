mod board;
mod engine;
mod pieces;
mod tests;
mod ui;

use crate::ui::menu::render;

use color_eyre::Result;

use crossterm::event::{self, Event, KeyCode};
use engine::Engine;
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
    loop {
        let engine = Engine::default();

        terminal.draw(|f| {
            let size = f.size();
            render::render_menu_ui(f, &engine, size);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Char('q') {
                    break Ok(());
                }
            }
        }
    }
}
