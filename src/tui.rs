mod command_popup;
mod file_content_provider;
mod input_handler;

use command_popup::*;
use file_content_provider::*;
use input_handler::*;

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Span;
use ratatui::widgets::{Borders, Clear};
use ratatui::{
    DefaultTerminal, Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, Paragraph},
};
use std::io;

pub fn execute() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default, Eq, PartialEq)]
enum Focused {
    #[default]
    TopLeft,
    TopRight,
    Bottom,
    PopUp,
}

#[derive(Debug, Default)]
pub struct App {
    focused: Focused,
    popup: CommandPopUpState,
    content_provider: FileContentProviderState,
    exit: bool,
    show_popup: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.popup = CommandPopUpState::new(vec!["snapshot".to_string(), "restore".to_string()]);
        self.content_provider = FileContentProviderState::new(vec![
            "DB Content".to_string(),
            "Snapshot Content".to_string(),
            "Executables Content".to_string(),
            "Ignore Content".to_string(),
        ]);
        while !self.exit {
            self.content_provider.read_files();
            terminal.draw(|frame| self.ui(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn ui(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.area());

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(outer_layout[0]);

        self.draw_left_upper(frame, inner_layout[0]);
        self.draw_right_upper(frame, inner_layout[1]);
        self.draw_bottom(frame, outer_layout[1]);
        if self.show_popup {
            self.popup.draw_popup(area, frame);
        }
    }

    fn draw_left_upper(&mut self, frame: &mut Frame, rect: Rect) {
        let lines = self.content_provider.provide_keys();
        let block = match self.focused {
            Focused::TopLeft => Block::new()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
            _ => Block::new().borders(Borders::ALL),
        };
        frame.render_widget(Paragraph::new(lines).block(block), rect);
    }

    fn draw_right_upper(&mut self, frame: &mut Frame, rect: Rect) {
        let text = self.content_provider.provide_values();
        let block = match self.focused {
            Focused::TopRight => Block::new()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
            _ => Block::new().borders(Borders::ALL),
        };
        frame.render_widget(Paragraph::new(text).block(block), rect);
    }

    fn draw_bottom(&self, frame: &mut Frame, rect: Rect) {
        let instructions = Line::from(vec![
            " Up/Down ".white().into(),
            "<Up>/<Down>".blue().bold(),
            " Switch focus ".white().into(),
            "<Tab>".blue().bold(),
            " Commands ".white().into(),
            "<R>".blue().bold(),
            " Quit ".white().into(),
            "<Q>".blue().bold(),
        ]);

        frame.render_widget(Clear, rect);
        let block = match self.focused {
            Focused::Bottom => Block::new()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title_bottom(instructions.centered()),
            _ => Block::new()
                .borders(Borders::ALL)
                .title_bottom(instructions.centered()),
        };
        frame.render_widget(Paragraph::new("outer 1").block(block), rect);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                pipeline_handle_key_event(self, key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
fn highlight_line(entry: &String) -> Line {
    Line::from(Span::styled(
        entry.clone(),
        Style::default()
            .fg(Color::White)
            .bg(Color::Blue)
            .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK),
    ))
}
