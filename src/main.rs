mod app;
mod area;
mod board;
mod controls;
mod pieces;
mod tests;
mod ui;

use app::{App, Screen};
use color_eyre::Result;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::crossterm::execute;
use ratatui::DefaultTerminal;

use crate::controls::key_events;
use ui::board::render_game_ui;
use ui::help::render_help_ui;
use ui::menu::render_menu_ui;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    result
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut app = App::default();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            match app.current_screen {
                Screen::Menu => render_menu_ui(f, &app, size),
                Screen::TwoPlayers | Screen::Bot => render_game_ui(f, size),
                Screen::Help => render_help_ui(f, size),
            }
        })?;

        if let Err(_) = key_events(&mut app) {
            break Ok(());
        }
    }
}
