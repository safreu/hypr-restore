use crate::tui::highlight_line;
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, BorderType, Clear, Paragraph};
use std::process::{Command, Stdio};

#[derive(Debug, Default)]
pub struct CommandPopUpState {
    selected_command: usize,
    commands: Vec<String>,
}

impl CommandPopUpState {
    pub fn new(commands: Vec<String>) -> Self {
        Self {
            selected_command: 0,
            commands,
        }
    }

    pub fn draw_popup(&mut self, area: Rect, frame: &mut Frame) {
        let block = Block::bordered()
            .title("Run command")
            .border_style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded);
        let lines: Vec<Line> = self
            .commands
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                if i == self.selected_command {
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

    pub fn move_up_commands(&mut self) {
        if self.selected_command > 0 {
            self.selected_command -= 1;
        } else {
            self.selected_command = self.commands.len() - 1;
        }
    }

    pub fn move_down_commands(&mut self) {
        if self.selected_command < self.commands.len() - 1 {
            self.selected_command += 1;
        } else {
            self.selected_command = 0;
        }
    }

    pub fn run_command(&mut self) {
        let args = match self.commands[self.selected_command].clone().as_str() {
            "snapshot" => Some(["run", "--bin", "snapshot"]),
            "restore" => Some(["run", "--bin", "restore"]),
            _ => return,
        };
        if let Some(args) = args {
            let mut cmd = Command::new("cargo");
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

