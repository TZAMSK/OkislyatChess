mod app;
mod board;
mod controls;
mod pieces;
mod tests;
mod ui;

use crate::controls::key_events;
use crate::ui::board::render_game_ui;
use app::{App, Screen};

use color_eyre::Result;
use ratatui::DefaultTerminal;
use ui::menu::render_menu_ui;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = App::default();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            match app.current_screen {
                Screen::Menu => render_menu_ui(f, &app, size),
                Screen::TwoPlayers | Screen::Bot => render_game_ui(f, size),
            }
        })?;

        if let Err(_) = key_events(&mut app) {
            break Ok(());
        }
    }
}
