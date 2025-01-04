use ratatui::style::Style;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::Rect,
    style::Stylize,
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

use crate::board::init_board;
use crate::ui::constants::{BOARD_BLACK, BOARD_LETTERS, BOARD_WHITE, PIECE_BLACK, PIECE_WHITE};

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
                .block(Block::new().padding(Padding::new(1, 1, 3, 1))),
            numbers_board_layout[index],
        );
    }

    // Board
    let board_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Ratio(1, 8); 8])
        .split(main_layout_vertical[2]);

    for (row_idx, row) in board_rows.iter().enumerate() {
        let row_cells = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 8); 8])
            .split(*row);

        for (col_idx, _) in row_cells.iter().enumerate() {
            let bg_color = if (row_idx + col_idx) % 2 == 1 {
                BOARD_BLACK
            } else {
                BOARD_WHITE
            };

            frame.render_widget(
                Paragraph::new("").block(Block::new().bg(bg_color)),
                row_cells[col_idx],
            );
        }
    }

    // Pieces
    let pieces = init_board();

    let pieces_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Ratio(1, 8); 8])
        .split(main_layout_vertical[2]);

    for (row_idx, row) in pieces_rows.iter().enumerate() {
        let row_cells = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 8); 8])
            .split(*row);

        for (col_idx, _) in row_cells.iter().enumerate() {
            let piece = &pieces[7 - row_idx][col_idx];

            let chess_piece = match piece {
                Some(p) => match p.color {
                    crate::pieces::Color::Black => p.draw().to_string(),
                    crate::pieces::Color::White => p.draw().to_string(),
                },
                None => "".to_string(),
            };

            let style = match piece {
                Some(p) => match p.color {
                    crate::pieces::Color::Black => Style::default().fg(PIECE_BLACK),
                    crate::pieces::Color::White => Style::default().fg(PIECE_WHITE),
                },
                None => Style::default(),
            };

            frame.render_widget(
                Paragraph::new(chess_piece)
                    .block(Block::new())
                    .alignment(Alignment::Center)
                    .style(style),
                row_cells[col_idx],
            );
        }
    }

    // Chance
    frame.render_widget(
        Paragraph::new("").block(Block::new().borders(Borders::ALL)),
        main_layout_vertical[3],
    );

    // Black feed
    frame.render_widget(
        Paragraph::new("").block(Block::new().borders(Borders::ALL)),
        right_box_layout[0],
    );

    // History
    frame.render_widget(
        Paragraph::new("").block(Block::new().borders(Borders::ALL)),
        right_box_layout[1],
    );

    // White feed
    frame.render_widget(
        Paragraph::new("").block(Block::new().borders(Borders::ALL)),
        right_box_layout[2],
    );
}
