use std::path::PathBuf;
use shared::event_entry::EventEntry;
use shared::file_handler::FileHandler;

/// An EventDB used to store the open window Events
pub struct EventDb {
    table: Vec<EventEntry>,
    file_handler: FileHandler,
}

impl EventDb {

    /// Constructs a new EventDb, based on the given Path
    ///
    /// # Arguments
    /// * `path` := The path used to construct a FileHandler for the DB
    ///
    /// # Returns
    /// Self
    pub fn new(path: PathBuf) -> Self {
        EventDb {
            table: Vec::new(),
            file_handler: FileHandler::new(path)
        }
    }

    /// Inserts an EventEntry to the EventDb
    ///
    /// # Arguments
    /// * `event` := EventEntry you want to add to the DB
    pub fn insert(&mut self, event: EventEntry) {
        let printable = event.to_string();
        self.table.push(event);
        self.file_handler.write(printable.as_str()).expect("Error writing to file");

    }

    /// Removes an EventEntry from to EventDb
    pub fn remove(&mut self, address: &str) -> std::io::Result<()> {
        if let Some(index) = self.get_index(address) {
            self.table.remove(index);
        }
        self.file_handler.remove_line(&mut self.table)
    }


    /// Gets the Index of an EventEntry by using the addres of the EvenEntry
    ///
    /// # Arguments
    /// * `address` := the address of the EventEntry used to get the index
    ///
    /// # Returns
    /// An Option of the index
    fn get_index(&self, address: &str) -> Option<usize> {
        let address = if !address.starts_with("0x") {
            format!("0x{}", address)
        } else {
            address.to_string()
        };
        self.table.iter().position(|entry| entry.address() == address)
    }
}