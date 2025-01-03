use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::Rect,
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

use crate::ui::constants::BOARD_LETTERS;

pub fn render_game_ui(frame: &mut Frame, main_area: Rect) {
    let main_layout_horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 25),
                Constraint::Ratio(2, 25),
                Constraint::Ratio(21, 25),
                Constraint::Ratio(1, 25),
            ]
            .as_ref(),
        )
        .split(main_area);

    let main_layout_vertical = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(6, 47),
                Constraint::Ratio(2, 47),
                Constraint::Ratio(24, 47), // Board
                Constraint::Ratio(1, 47),
                Constraint::Ratio(2, 47),
                Constraint::Ratio(9, 47),
                Constraint::Ratio(3, 47),
            ]
            .as_ref(),
        )
        .split(main_layout_horizontal[2]);

    let right_box_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(3, 21),
                Constraint::Ratio(15, 21),
                Constraint::Ratio(3, 21),
            ]
            .as_ref(),
        )
        .split(main_layout_vertical[5]);

    let letter_board_main_box_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(8, 47),
                Constraint::Ratio(24, 47),
                Constraint::Ratio(15, 47),
            ]
            .as_ref(),
        )
        .split(main_layout_horizontal[1]);

    let letters_board_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(3, 24),
                Constraint::Ratio(3, 24),
                Constraint::Ratio(3, 24),
                Constraint::Ratio(3, 24),
                Constraint::Ratio(3, 24),
                Constraint::Ratio(3, 24),
                Constraint::Ratio(3, 24),
                Constraint::Ratio(3, 24),
            ]
            .as_ref(),
        )
        .split(letter_board_main_box_layout[1]);

    let numbers_board_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(3, 24),
                Constraint::Ratio(3, 24),
                Constraint::Ratio(3, 24),
                Constraint::Ratio(3, 24),
                Constraint::Ratio(3, 24),
                Constraint::Ratio(3, 24),
                Constraint::Ratio(3, 24),
                Constraint::Ratio(3, 24),
            ]
            .as_ref(),
        )
        .split(main_layout_vertical[1]);

    // Letters
    for (index, letter) in BOARD_LETTERS.iter().enumerate() {
        frame.render_widget(
            Paragraph::new(*letter)
                .alignment(Alignment::Center)
                .block(Block::new().padding(Padding::new(1, 1, 2, 1))),
            letters_board_layout[index],
        );
    }

    // Numbers
    for (index, number) in (1..=8).rev().enumerate() {
        frame.render_widget(
            Paragraph::new(number.to_string())
                .alignment(Alignment::Center)
                .block(Block::new().padding(Padding::new(1, 1, 2, 1))),
            numbers_board_layout[index],
        );
    }

    // Chance
    frame.render_widget(
        Paragraph::new("a block3").block(Block::new().borders(Borders::ALL)),
        main_layout_vertical[2],
    );

    // Board
    frame.render_widget(
        Paragraph::new("a block4").block(Block::new().borders(Borders::ALL)),
        main_layout_vertical[3],
    );

    // Black feed
    frame.render_widget(
        Paragraph::new("a block9").block(Block::new().borders(Borders::ALL)),
        right_box_layout[0],
    );

    // History
    frame.render_widget(
        Paragraph::new("a block10").block(Block::new().borders(Borders::ALL)),
        right_box_layout[1],
    );

    // White feed
    frame.render_widget(
        Paragraph::new("a block11").block(Block::new().borders(Borders::ALL)),
        right_box_layout[2],
    );
}
