use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::Rect,
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

use super::constants::{HELP, PIECES};

pub fn render_help_ui(frame: &mut Frame, main_area: Rect) {
    let main_layout_horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(2, 15),
                Constraint::Ratio(1, 15),
                Constraint::Ratio(11, 15),
                Constraint::Ratio(1, 15),
            ]
            .as_ref(),
        )
        .split(main_area);

    let main_layout_vertical = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(1, 20),
                Constraint::Ratio(3, 20),
                Constraint::Ratio(3, 20),
                Constraint::Ratio(3, 20),
                Constraint::Ratio(3, 20),
                Constraint::Ratio(3, 20),
                Constraint::Ratio(3, 20),
                Constraint::Ratio(1, 20),
            ]
            .as_ref(),
        )
        .split(main_layout_horizontal[2]);

    let main_piece_names_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(1, 20),
                Constraint::Ratio(18, 20),
                Constraint::Ratio(1, 20),
            ]
            .as_ref(),
        )
        .split(main_layout_horizontal[1]);

    frame.render_widget(
        Paragraph::new(HELP)
            .alignment(Alignment::Center)
            .block(Block::new().borders(Borders::ALL)),
        main_layout_horizontal[0],
    );

    // Names
    let piece_names_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(3, 18),
                Constraint::Ratio(3, 18),
                Constraint::Ratio(3, 18),
                Constraint::Ratio(3, 18),
                Constraint::Ratio(3, 18),
                Constraint::Ratio(3, 18),
            ]
            .as_ref(),
        )
        .split(main_piece_names_layout[1]);

    for (index, name) in PIECES.iter().enumerate() {
        frame.render_widget(
            Paragraph::new(*name)
                .alignment(Alignment::Center)
                .block(Block::new().borders(Borders::ALL)),
            piece_names_layout[index],
        );
    }

    // Piece description
    for (i, _) in (1..=8).enumerate() {
        frame.render_widget(
            Paragraph::new("").block(Block::new().borders(Borders::ALL)),
            main_layout_vertical[i],
        );
    }
}
