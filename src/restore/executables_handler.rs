use std::collections::HashMap;
use std::{fs, io};
use std::io::Error;
use std::path::PathBuf;
use crate::shared::event_entry::EventEntry;
use crate::shared::file_handler::FileHandler;

/// Contains the table which saves class \[KEY] and path \[VALUE] and the fileHandler to write them to in shared specified path
pub struct ExecutablesHandler {
    pub(crate) file_handler: FileHandler,
    pub(crate) table: HashMap<String, String>
}

impl ExecutablesHandler {
    /// Creates a new ExecutablesMap by instantiating the fileHandler and the table
    pub fn new(path: PathBuf) -> Self {
        Self {
            file_handler: FileHandler::new(path),
            table: HashMap::new()
        }
    }
    /// Initializes the table inside of Executables, by reading the file which contains the paths and writing them into the table,
    /// # Warning
    /// **Must** be called **after** new()
    pub fn init(&mut self) {
        let lines = self.file_handler
            .read_file()
            .expect(format!("No Executables in {}", self.file_handler.get_path().trim()).as_str());
        for line in lines {
            let split = EventEntry::split(&line);
            self.table.insert(split[0].clone(), split[1].clone());
        }
    }

    /// Inserts the class and the path to the table,
    /// also writes it into the file if the table changes
    ///
    /// # Arguments
    /// * `class` := the class for the application \[KEY]
    /// * `executable_path` := the path to execute the application \[VALUE], can be determined with get_executable_path_from_env
    ///
    /// # Return
    /// io::Result based on if the operation was successfully or not
    #[allow(dead_code)]
    pub fn insert(&mut self, class: String, executable_path: String) -> io::Result<()> {
        if self.table.contains_key(&class) && self.table[&class] == executable_path { return Ok(()); }
        else { self.table.insert(class.clone(), executable_path.clone()); }
        self.file_handler.write_complete_hashmap(&self.table)
    }

    /// Gets the Executable Path fromm the environment
    ///
    /// # Arguments
    /// `address` := address Hyprland distributes
    ///
    /// # Returns
    /// the Path to execute the application
    #[allow(dead_code)]
    pub fn get_executable_path_from_env(address: &str) -> Result<String, Error> {
        let pid = crate::shared::get_pid(address);
        let path = fs::read_link(format!("/proc/{}/exe", pid))?;
        Ok(path.to_string_lossy().into_owned())
    }

    /// Gets the executable path in the table
    ///
    /// # Arguments
    /// `class` := the class of the application
    ///
    /// # Returns
    /// The Option of the entry
    pub fn get_executable_entry(&mut self, class: &str) -> Option<String> {
        if self.table.contains_key(class) {
            return Some(self.table[class].to_string())
        }
        None
    }
}
