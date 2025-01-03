mod board;
mod controls;
mod engine;
mod pieces;
mod tests;
mod ui;

use crate::controls::key_events;
use crate::ui::menu;
use engine::Engine;

use color_eyre::Result;
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
            menu::render_menu_ui(f, &engine, size);
        })?;

        if let Err(_) = key_events(&mut engine) {
            break Ok(());
        }
    }
}
