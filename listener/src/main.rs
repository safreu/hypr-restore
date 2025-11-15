mod instance_handler;
mod event_db;
mod ignored_handler;

use std::io::{BufRead};
use tracing_subscriber::EnvFilter;
use log::info;
use restore::executables_handler::ExecutablesHandler;
use crate::event_db::EventDb;
use shared::event_entry::EventEntry;
use crate::ignored_handler::IgnoredHandler;
use crate::instance_handler::InstanceHandler;

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .with_level(true)
        .init();

    let ignored_handler = IgnoredHandler::new(shared::ignore_path());
    let mut table: EventDb = EventDb::new(shared::db_path());
    let instance = InstanceHandler::new();
    let mut executables_handler = ExecutablesHandler::new(shared::executables_path());
    executables_handler.init();

    for line in instance.reader().lines() {
        match line {
            Ok(line) => {
                let parts = EventEntry::split(&line);

                if line.contains("openwindow") {
                    info!("Opened Window: {}", line);
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
                    info!("Closed Window: {}", line);
                    table.remove(&parts[1]).expect("Failed to remove entry");
                }
            }
            Err(_) => continue,
        }
    }
    Ok(())
}
