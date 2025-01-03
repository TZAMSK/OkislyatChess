use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

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
                Constraint::Ratio(6, 45),
                Constraint::Ratio(2, 45),
                Constraint::Ratio(22, 45), // Board
                Constraint::Ratio(1, 45),
                Constraint::Ratio(2, 45),
                Constraint::Ratio(9, 45),
                Constraint::Ratio(3, 45),
            ]
            .as_ref(),
        )
        .split(main_layout_horizontal[2]);

    let right_box_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(2, 15),
                Constraint::Ratio(11, 15),
                Constraint::Ratio(2, 15),
            ]
            .as_ref(),
        )
        .split(main_layout_vertical[5]);

    let letter_board_box_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(8, 45),
                Constraint::Ratio(22, 45),
                Constraint::Ratio(13, 45),
            ]
            .as_ref(),
        )
        .split(main_layout_horizontal[1]);

    // Letters
    frame.render_widget(
        Paragraph::new("a block8").block(Block::new().borders(Borders::ALL)),
        letter_board_box_layout[1],
    );

    // Numbers
    frame.render_widget(
        Paragraph::new("a").block(Block::new().borders(Borders::ALL)),
        main_layout_vertical[1],
    );

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
