mod instance_handler;
mod event_db;
mod ignored_handler;
mod event_validation;
use std::io::{BufRead};
use tracing_subscriber::EnvFilter;
use log::info;
use shared::event_entry::EventEntry;
use shared::executables_path;
use crate::instance_handler::InstanceHandler;
use crate::event_validation::EventValidation;

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .with_level(true)
        .init();

    let instance = InstanceHandler::new();
    let mut event_validator = EventValidation::new(shared::db_path(), shared::ignore_path(), executables_path());

    for line in instance.reader().lines() {
        match line {
            Ok(line) => {
                let parts = EventEntry::split(&line);
                if line.contains("openwindow") {
                    info!("Opened Window: {}", line);
                    let event: EventEntry = EventEntry::new(&parts);
                    let _ = event_validator.try_insert(event);
                }

                if line.contains("movewindowv2") {
                    info!("Moved Window: {}", line);
                    let _ = event_validator.try_update_workspace(&parts[1], &parts[2]);
                }

                if line.contains("closewindow") {
                    info!("Closed Window: {}", line);
                    let _ = event_validator.try_remove(&parts[1]);
                }
            }
            Err(_) => continue,
        }
    }
    Ok(())
}
