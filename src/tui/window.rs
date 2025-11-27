use crate::tui::{App, Tab};
use ansi_to_tui::IntoText;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::text::Text;
use ratatui::widgets::Borders;
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Paragraph},
};
use std::process::Command;

pub fn draw_windows(app: &mut App, frame: &mut Frame, rect: Rect) {
    match app.tab_state.active_tab() {
        Tab::FileContent => render_file_content_windows(app, frame, rect),
        Tab::ListenerService => render_listener_service_windows(frame, rect),
        Tab::RestoreService => render_restore_service_windows(frame, rect),
        Tab::CustomWorkspace => render_custom_workspaces_windows(frame, rect),
    }
}

fn render_file_content_windows(app: &mut App, frame: &mut Frame, rect: Rect) {
    let [left, right] = create_horizontal_layout(rect);

    let left_block = Block::new().borders(Borders::ALL);
    let right_block = Block::new().borders(Borders::ALL);

    let keys = app.content_provider.provide_keys();
    let values = app.content_provider.provide_values();

    frame.render_widget(Paragraph::new(keys).block(left_block), left);
    frame.render_widget(Paragraph::new(values).block(right_block), right);
}

fn render_listener_service_windows(frame: &mut Frame, rect: Rect) {
    let [upper, lower] = create_vertical_layout(rect);

    let upper_block = Block::new().borders(Borders::ALL);

    let lower_block = Block::new().borders(Borders::ALL);

    frame.render_widget(
        Paragraph::new(run_systemctl("hypr-listener")).block(upper_block),
        upper,
    );

    frame.render_widget(
        Paragraph::new(Text::raw(run_journalctl("hypr-listener"))).block(lower_block),
        lower,
    );
}

fn render_restore_service_windows(frame: &mut Frame, rect: Rect) {
    let [upper, lower] = create_vertical_layout(rect);

    let upper_block = Block::new().borders(Borders::ALL);

    let lower_block = Block::new().borders(Borders::ALL);

    frame.render_widget(
        Paragraph::new(run_systemctl("hypr-snapshot")).block(upper_block),
        upper,
    );

    frame.render_widget(
        Paragraph::new(Text::raw(run_journalctl("hypr-snapshot"))).block(lower_block),
        lower,
    );
}

fn render_custom_workspaces_windows(frame: &mut Frame, rect: Rect) {
    let [left, right] = create_horizontal_layout(rect);

    let left_block = Block::new().borders(Borders::ALL);

    let right_block = Block::new().borders(Borders::ALL);

    frame.render_widget(left_block, left);
    frame.render_widget(right_block, right);
}

fn create_vertical_layout(rect: Rect) -> [ratatui::layout::Rect; 2] {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rect);

    let [upper_layout, lower_layout] = *layout else {
        panic!("Cant create Layouts")
    };

    [upper_layout, lower_layout]
}

fn create_horizontal_layout(rect: Rect) -> [ratatui::layout::Rect; 2] {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rect);

    let [left_layout, right_layout] = *layout else {
        panic!("Cant create Layouts")
    };

    [left_layout, right_layout]
}

fn run_systemctl(service_name: &str) -> Text<'_> {
    let systemctl_output = Command::new("systemctl")
        .args(["--user", "status", service_name])
        .output()
        .expect("Failed to run systemctl");

    let converted_output = String::from_utf8_lossy(&systemctl_output.stdout);
    let header = converted_output.split("\n\n").next().unwrap_or("");

    header
        .as_bytes()
        .into_text()
        .unwrap_or_else(|_| Text::raw("failed to parse"))
}

fn run_journalctl(service_name: &str) -> std::string::String {
    let journalctl_output = Command::new("journalctl")
        .args(["-n", "10", "--no-pager", "--user", "-u", service_name])
        .output()
        .expect("Failed to run journalctl");

    String::from_utf8_lossy(&journalctl_output.stdout).to_string()
}
