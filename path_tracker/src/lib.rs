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

struct ExecutablesMap {
    file_handler: FileHandler,
    table: HashMap<String, String>
}

impl ExecutablesMap {
    fn new() -> Self {
        Self {
            file_handler: FileHandler::new(shared::EXECUTABLE_PATH.to_string()),
            table: HashMap::new()
        }
    }
    fn init(&mut self) {
        let lines = self.file_handler.read_file().expect("Failed to read file");
        for line in lines {
            let split = EventEntry::split(&line);
            self.table.insert(split[0].clone(), split[1].clone());
        }
    }

    fn insert(&mut self, class: String, path: String) -> io::Result<()> {
        if self.table.contains_key(&class) && self.table[&class] == path { return Ok(()); }
        else { self.table.insert(class.clone(), path.clone()); }
        self.file_handler.write_complete_hashmap(&self.table)
    }

    fn get_executable_path_from_env(address: &str) -> Result<String, Error> {
        let pid = utils::get_pid(address);
        utils::get_env_value("_=", &pid)
    }
    
    fn get_executable_entry(&mut self, class: &str) -> Option<String> {
        if self.table.contains_key(class) {
            return Some(self.table[class].to_string())
        } 
        None
    }
}
