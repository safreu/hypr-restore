use std::path::{PathBuf};
use restore::executables_handler::ExecutablesHandler;
use shared::event_entry::EventEntry;
use crate::event_db::EventDb;
use crate::ignored_handler::IgnoredHandler;

/// The EventValidation validates events and based on the validation performs operations on the DB
pub struct EventValidation {
    db : EventDb,
    ignored_handler: IgnoredHandler,
    executables_handler: ExecutablesHandler,
}

impl EventValidation {
    
    /// Constructs a new EventValidation
    /// 
    /// # Arguments
    /// * `db_path` := The path to the DB on which to perform the operations
    /// * `ignore_path` := The path to the file which describes what to ignore
    /// * `executables_path` := The path to the file which stores the paths to the executables
    /// 
    /// # Returns
    /// Self
    pub fn new(db_path: PathBuf, ignore_path: PathBuf, executables_path: PathBuf) -> Self {
        let mut executables_handler = ExecutablesHandler::new(executables_path);
        executables_handler.init();
        Self {
            db: EventDb::new(db_path),
            ignored_handler: IgnoredHandler::new(ignore_path),
            executables_handler,
        }
    }

    /// Tries to insert the given Event, based on the validations
    /// 
    /// # Arguments
    /// * `event` := The event to insert
    /// 
    /// # Returns
    /// If successfully an empty Result or the EventEntry
    pub fn try_insert(&mut self, event: EventEntry) -> Result<(), EventEntry> {
        if self.ignored_handler.should_ignore(event.class(), event.address()) { return Err(event) }

        match ExecutablesHandler::get_executable_path_from_env(&event.address()) {
            Ok(executable_path) => self.executables_handler
                .insert(
                    event.class().to_string(),
                    executable_path).expect("Failed to insert executable path"),
            Err(_) => {self.executables_handler
                .insert(
                    event.class().to_string(),
                    event.class().to_string()).expect("Failed to insert class");}
        }

        self.db.insert(event);
        Ok(())
    }

    /// Tries to remove an Event
    /// 
    /// # Arguments
    /// * `address` := The address you want to remove
    ///
    /// # Returns
    /// If successfully an empty Result or the address
    pub fn try_remove(&mut self, address: &str) -> Result<(), String> {
        match self.db.remove(address) {
            Ok(_) => {Ok(())}
            Err(_) => {Err(address.to_string()) }
        }
    }

    /// Tries to update the workspace of an Event
    /// 
    /// # Arguments
    /// * `address` := The address of the Event you want to update
    /// * `workspace` := The workspace to which you want to update
    /// 
    /// # Returns
    /// An Result containing the event if the operation was successfully, else an empty Result
    pub fn try_update(&mut self, address: &str, workspace: &str) -> Result<EventEntry, ()> {
        let mut modified_address = address.to_string();
        if !modified_address.starts_with("0x") { modified_address = format!("0x{}", address) }
        match self.db.update_workspace(&modified_address, workspace) {
            Some(e) => Ok(e),
            None => Err(())
        }
    }
}