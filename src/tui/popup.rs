use crate::tui::highlight_line;
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, BorderType, Clear, Paragraph};
use std::process::{Command, Stdio};

#[derive(Debug, Default, PartialEq)]
pub struct Popup<B: PopupBehavior> {
    selected_entry: usize,
    entries: Vec<String>,
    behavior: B,
}

pub trait PopupBehavior: Sized {
    fn run_command(&self, popup: &Popup<Self>);
}

impl<B: PopupBehavior> Popup<B> {
    pub fn draw_popup(&self, area: Rect, frame: &mut Frame) {
        let block = Block::bordered()
            .title("Run command")
            .border_style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded);
        let lines: Vec<Line> = self
            .entries
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                if i == self.selected_entry {
                    highlight_line(entry)
                } else {
                    Line::from(Span::raw(entry.clone()))
                }
            })
            .collect();
        let content = Paragraph::new(lines).block(block);
        let area = popup_area(area, 60, 20);
        frame.render_widget(Clear, area);
        frame.render_widget(content, area);
    }

    pub fn run_command(&mut self) {
        self.behavior.run_command(self);
    }

    pub fn new(entries: Vec<String>, behavior: B) -> Self {
        Self {
            selected_entry: 0,
            entries,
            behavior,
        }
    }

    pub fn move_up_entries(&mut self) {
        if self.selected_entry > 0 {
            self.selected_entry -= 1;
        } else {
            self.selected_entry = self.entries.len() - 1;
        }
    }

    pub fn move_down_entries(&mut self) {
        if self.selected_entry < self.entries.len() - 1 {
            self.selected_entry += 1;
        } else {
            self.selected_entry = 0;
        }
    }
}

#[derive(Default, PartialEq, Debug)]
pub struct FileContentPopUp;

impl PopupBehavior for FileContentPopUp {
    fn run_command(&self, popup: &Popup<Self>) {
        let args = match popup.entries[popup.selected_entry].clone().as_str() {
            "snapshot" => Some(["snapshot"]),
            "restore" => Some(["restore"]),
            _ => return,
        };
        if let Some(args) = args {
            let mut cmd = Command::new("hypr-restore");
            cmd.args(args)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .stdin(Stdio::null());

            if let Err(e) = cmd.spawn() {
                eprintln!("Failed to run command: {e}");
            }
        }
    }
}

#[derive(Default, PartialEq, Debug)]
pub struct ListenerServicePopUp;

impl PopupBehavior for ListenerServicePopUp {
    fn run_command(&self, popup: &Popup<Self>) {
        let args = match popup.entries[popup.selected_entry].clone().as_str() {
            "start" => Some(["--user", "start", "hypr-listener"]),
            "restart" => Some(["--user", "restart", "hypr-listener"]),
            "enable" => Some(["--user", "enable", "hypr-listener"]),
            "disable" => Some(["--user", "disable", "hypr-listener"]),
            "stop" => Some(["--user", "stop", "hypr-listener"]),
            _ => return,
        };
        if let Some(args) = args {
            let mut cmd = Command::new("systemctl");
            cmd.args(args)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .stdin(Stdio::null());

            if let Err(e) = cmd.spawn() {
                eprintln!("Failed to run command: {e}");
            }
        }
    }
}

#[derive(Default, PartialEq, Debug)]
pub struct SnapshotServicePopUp;

impl PopupBehavior for SnapshotServicePopUp {
    fn run_command(&self, popup: &Popup<Self>) {
        let args = match popup.entries[popup.selected_entry].clone().as_str() {
            "start" => Some(["--user", "start", "hypr-snapshot"]),
            "restart" => Some(["--user", "restart", "hypr-snapshot"]),
            "enable" => Some(["--user", "enable", "hypr-snapshot"]),
            "disable" => Some(["--user", "disable", "hypr-snapshot"]),
            "stop" => Some(["--user", "stop", "hypr-snapshot"]),
            _ => return,
        };
        if let Some(args) = args {
            let mut cmd = Command::new("systemctl");
            cmd.args(args)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .stdin(Stdio::null());

            if let Err(e) = cmd.spawn() {
                eprintln!("Failed to run command: {e}");
            }
        }
    }
}
fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
