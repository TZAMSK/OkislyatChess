use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::app::App;
use crate::pieces::Color;
use crate::ui::constants::{MENU_ITEMS, TITLE};

pub fn render_game_ui(frame: &mut Frame<'_>, app: &mut App, main_area: Rect) {
    let main_layout_horizontal = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 18),
                Constraint::Ratio(16, 18),
                Constraint::Ratio(1, 18),
            ]
            .as_ref(),
        )
        .split(main_area);

    let main_layout_vertical = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(2, 17),
                Constraint::Ratio(9, 17),
                Constraint::Ratio(1, 17),
                Constraint::Ratio(5, 17),
            ]
            .as_ref(),
        )
        .split(main_layout_horizontal[1]);

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
        .split(main_layout_vertical[3]);

    // We render the board_block in the center layout made above
    frame.render_widget(board_block.clone(), main_layout_vertical[1]);

    let game_clone = app.game.clone();

    app.game.ui.board_render(
        board_block.inner(main_layout_vertical[1]),
        frame,
        &game_clone,
    ); // Mutable borrow now allowed

    //top box for white material
    app.game.ui.black_material_render(
        app.inner(right_box_layout[0]),
        frame,
        &app.game.game_board.black_taken_pieces,
    );

    // We make the inside of the board
    app.game.ui.history_render(
        board_block.inner(right_box_layout[1]),
        frame,
        &app.game.game_board.move_history,
    );

    //bottom box for black matetrial
    app.game.ui.white_material_render(
        board_block.inner(right_box_layout[2]),
        frame,
        &app.game.game_board.white_taken_pieces,
    );
}
