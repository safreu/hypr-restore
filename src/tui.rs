mod popup;
mod file_content_provider;
mod input_handler;
mod navigation_bar;
mod tab;
mod window;

use crossterm::event::{self, Event, KeyEventKind};
use file_content_provider::*;
use input_handler::*;
use navigation_bar::draw_navigation_bar;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Span;
use ratatui::text::{Line};
use ratatui::widgets::{Borders};
use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Block},
};

use std::io;
use tab::Tab;

use crate::tui::tab::TabState;
use crate::tui::window::draw_windows;

pub fn execute() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    tab_state: TabState,
    content_provider: FileContentProvider,
    exit: bool,
    show_popup: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.tab_state = TabState::new();
        self.content_provider = FileContentProvider::new(vec![
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

        let instructions = Line::from(vec![
            " Up/Down ".white(),
            "<Up>/<Down>".blue().bold(),
            " Switch Tab ".white(),
            "<Tab>".blue().bold(),
            " Commands ".white(),
            "<R>".blue().bold(),
            " Quit ".white(),
            "<Q>".blue().bold(),
        ]);

        let screen = Block::new()
            .borders(Borders::NONE)
            .title_bottom(instructions.centered());

        let split_screen = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(7), Constraint::Percentage(93)])
            .split(area);

        let [navigation_layout, content_layout] = &*split_screen else {
            panic!("Cant create Layouts")
        };

        draw_navigation_bar(self, frame, *navigation_layout);
        draw_windows(self, frame, *content_layout);
        if self.show_popup {
            self.tab_state.draw_popup(area, frame);
        }
        frame.render_widget(screen, area)
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

fn highlight_line(entry: &str) -> Line {
    Line::from(Span::styled(
        entry,
        Style::default()
            .fg(Color::Black)
            .bg(Color::Blue)
            .add_modifier(Modifier::BOLD | Modifier::RAPID_BLINK),
    ))
}
