use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::engine::Engine;
use crate::ui::constants::TITLE;

pub fn render_menu_ui(frame: &mut Frame, engine: &Engine, main_area: Rect) {
    let main_layout_horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 5),
                Constraint::Ratio(1, 5),
                Constraint::Ratio(3, 5),
            ]
            .as_ref(),
        )
        .split(main_area);

    let title_paragraph = Paragraph::new(TITLE)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(title_paragraph, main_layout_horizontal[0]);

    let text: Vec<Line<'_>> = vec![Line::from(""), Line::from("A chess game made in 🦀")];
    let sub_title = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(sub_title, main_layout_horizontal[1]);

    let menu_items = ["Solo", "Bot", "Quit"];
    let mut menu_body: Vec<Line<'_>> = vec![];

    for (i, menu_item) in menu_items.iter().enumerate() {
        menu_body.push(Line::from(""));
        let mut text = if engine.menu_cursor == i as u8 {
            "> ".to_string()
        } else {
            String::new()
        };
        text.push_str(menu_item);
        menu_body.push(Line::from(text));
    }

    let sub_title = Paragraph::new(menu_body)
        .bold()
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(sub_title, main_layout_horizontal[2]);
}
