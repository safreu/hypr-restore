mod tests;
//TODO: get Address -> use it to get PID -> use it to get PATH
//TODO: Write Path to file to save it
//TODO: validate if Path is already in File -> Skip if exists || Write if not
//TODO: later use it to execute the applications in restore
//TODO: much later find solution if executing them over path is not desired
use std::collections::HashMap;
use std::io;
use std::io::Error;
use listener::event_entry::EventEntry;
use listener::file_handler::FileHandler;

/// Contains the table which saves class \[KEY] and path \[VALUE] and the fileHandler to write them to in shared specified path
struct ExecutablesMap {
    file_handler: FileHandler,
    table: HashMap<String, String>
}

impl ExecutablesMap {
    /// Creates a new ExecutablesMap by instantiating the fileHandler and the table
    fn new(path: &str) -> Self {
        Self {
            file_handler: FileHandler::new(path.to_string()),
            table: HashMap::new()
        }
    }
    /// Initializes the table inside of Executables, by reading the file which contains the paths and writing them into the table,
    /// # Warning
    /// **Must** be called **after** new()
    fn init(&mut self) {
        let lines = self.file_handler.read_file().expect("Failed to read file");
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
    fn insert(&mut self, class: String, executable_path: String) -> io::Result<()> {
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
    fn get_executable_path_from_env(address: &str) -> Result<String, Error> {
        let pid = utils::get_pid(address);
        utils::get_env_value("_=", &pid)
    }

    /// Gets the executable path in the table
    ///
    /// # Arguments
    /// `class` := the class of the application
    ///
    /// # Returns
    /// The Option of the entry
    fn get_executable_entry(&mut self, class: &str) -> Option<String> {
        if self.table.contains_key(class) {
            return Some(self.table[class].to_string())
        }
        None
    }
}
