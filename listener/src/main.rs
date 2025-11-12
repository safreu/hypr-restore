mod instance_handler;
mod event_db;
mod ignored_handler;

use std::io::{BufRead};
use restore::executables_handler::ExecutablesHandler;
use shared::{DB_PATH, IGNORE_PATH};
use crate::event_db::EventDb;
use shared::event_entry::EventEntry;
use crate::ignored_handler::IgnoredHandler;
use crate::instance_handler::InstanceHandler;

fn main() -> std::io::Result<()> {
    let ignored_handler = IgnoredHandler::new(IGNORE_PATH);
    let mut table: EventDb = EventDb::new(DB_PATH.to_string());
    let instance = InstanceHandler::new();
    let mut executables_handler = ExecutablesHandler::new(shared::EXECUTABLE_PATH);
    executables_handler.init();

    for line in instance.reader().lines() {
        match line {
            Ok(line) => {
                let parts = EventEntry::split(&line);

                if line.contains("openwindow") {
                    let event: EventEntry = EventEntry::new(&parts);
                    match ExecutablesHandler::get_executable_path_from_env(&event.address()) {
                        Ok(executable_path) => executables_handler.insert(event.class().to_string(), executable_path)?,
                        Err(_) => {executables_handler.insert(event.class().to_string(), event.class().to_string())?;}
                    }
                    if !ignored_handler.should_ignore(event.class(), event.address()) {
                        table.insert(event);
                    }
                }

                if line.contains("closewindow") {
                    table.remove(&parts[1]).expect("Failed to remove entry");
                }
            }
            Err(_) => continue,
        }
    }
    Ok(())
}
