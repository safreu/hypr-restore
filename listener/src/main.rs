mod event_entry;
mod instance_handler;
mod event_db;
mod file_handler;
mod ignored_handler;

use std::io::{BufRead};
use crate::event_db::EventDb;
use crate::event_entry::EventEntry;
use crate::ignored_handler::IgnoredHandler;
use crate::instance_handler::InstanceHandler;

fn main() -> std::io::Result<()> {
    let ignored_handler = IgnoredHandler::new("ignore.txt");
    let mut table: EventDb = EventDb::new("log.txt".to_string());
    let instance = InstanceHandler::new();

    for line in instance.reader().lines() {
        match line {
            Ok(line) => {
                let parts: Vec<String> = EventEntry::split(&line);
                
                if line.contains("openwindow") && !ignored_handler.should_ignore(&parts[3], &parts[1]) {
                    table.insert(
                        EventEntry::open_window(parts[1].clone(), parts[2].clone(), parts[3].clone(), parts[4].clone())
                    );
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
