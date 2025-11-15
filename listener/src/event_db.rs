use std::path::PathBuf;
use shared::event_entry::EventEntry;
use shared::file_handler::FileHandler;

pub struct EventDb {
    table: Vec<EventEntry>,
    file_handler: FileHandler,
}

impl EventDb {
    pub fn new(path: PathBuf) -> Self {
        EventDb {
            table: Vec::new(),
            file_handler: FileHandler::new(path)
        }
    }

    pub fn insert(&mut self, event: EventEntry) {
        let printable = event.to_string();
        self.table.push(event);
        self.file_handler.write(printable.as_str()).expect("Error writing to file");

    }

    pub fn remove(&mut self, address: &str) -> std::io::Result<()> {
        if let Some(index) = self.get_index(address) {
            self.table.remove(index);
        }
        self.file_handler.remove_line(&mut self.table)
    }

    fn get_index(&self, address: &str) -> Option<usize> {
        let address = if !address.starts_with("0x") {
            format!("0x{}", address)
        } else {
            address.to_string()
        };
        self.table.iter().position(|entry| entry.address() == address)
    }
}