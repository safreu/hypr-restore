use std::collections::HashMap;
use ratatui::prelude::{Line, Span, Text};
use shared::{DB_PATH, EXECUTABLE_PATH, IGNORE_PATH, SNAPSHOT_PATH};
use shared::file_handler::FileHandler;
use crate::highlight_line;

#[derive(Debug, Default)]
pub struct FileContentProviderState {
    content_list: Vec<String>,
    content_entries: HashMap<String, Vec<String>>,
    content_highlight: usize,
}

impl FileContentProviderState {
    pub fn new(content_list: Vec<String>) -> Self {
        Self {
            content_list,
            content_entries: HashMap::new(),
            content_highlight: 0,
        }
    }

    pub fn read_files(&mut self) {
        let db_handler = FileHandler::new(DB_PATH.to_string());
        let snapshot_handler = FileHandler::new(SNAPSHOT_PATH.to_string());
        let executables_handler = FileHandler::new(EXECUTABLE_PATH.to_string());
        let ignore_handler = FileHandler::new(IGNORE_PATH.to_string());

        self.content_entries.clear();
        self.content_entries.insert("DB Content".to_string(), db_handler.read_file_as_vec().unwrap());
        self.content_entries.insert("Snapshot Content".to_string(), snapshot_handler.read_file_as_vec().unwrap());
        self.content_entries.insert("Executables Content".to_string(), executables_handler.read_file_as_vec().unwrap());
        self.content_entries.insert("Ignore Content".to_string(), ignore_handler.read_file_as_vec().unwrap());
    }

    pub fn provide_keys(&mut self) -> Vec<Line> {
        self.content_list
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                if i == self.content_highlight {
                    highlight_line(entry)
                } else {
                    Line::from(Span::raw(entry.clone()))
                }
            }).collect()
    }

    pub fn provide_values(&mut self) -> Text {
        let selected_command = self.content_list[self.content_highlight].clone();
        let selected_entries = self.content_entries.get(&selected_command).unwrap();
        Text::from(selected_entries.join("\n"))
    }

    pub fn move_up_contents(&mut self) {
        if self.content_highlight > 0 {
            self.content_highlight -= 1;
        } else {
            self.content_highlight = self.content_list.len() - 1;
        }
    }

    pub fn move_down_contents(&mut self) {
        if self.content_highlight < self.content_list.len() -1 {
            self.content_highlight += 1;
        } else {
            self.content_highlight = 0;
        }
    }
}