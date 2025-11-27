use crate::tui::{App, Tab, highlight_line};

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::text::Line;
use ratatui::widgets::Borders;
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Paragraph},
};

pub fn draw_navigation_bar(app: &mut App, frame: &mut Frame, rect: Rect) {
    let block = Block::new().borders(Borders::ALL);
    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(rect);
    draw_navigation_options(
        app,
        frame,
        inner_layout[0],
        "File Content",
        Tab::FileContent,
    );
    draw_navigation_options(
        app,
        frame,
        inner_layout[1],
        "hypr-listener.service",
        Tab::ListenerService,
    );
    draw_navigation_options(
        app,
        frame,
        inner_layout[2],
        "hypr-restore.service",
        Tab::RestoreService,
    );
    draw_navigation_options(
        app,
        frame,
        inner_layout[3],
        "Custom Workspaces",
        Tab::CustomWorkspace,
    );

    frame.render_widget(block, rect);
}

fn draw_navigation_options(
    app: &mut App,
    frame: &mut Frame,
    rect: Rect,
    text: &str,
    calling_tab: Tab,
) {
    let block = Block::new().borders(Borders::ALL);

    let line: Line = if calling_tab == *app.tab_state.active_tab() {
        highlight_line(text).centered()
    } else {
        Line::from(text).centered()
    };

    frame.render_widget(Paragraph::new(line).block(block), rect);
}
