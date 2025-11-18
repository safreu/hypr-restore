use std::fs;
use std::process::Command;
use crate::restore::executables_handler::ExecutablesHandler;
use crate::shared;
use crate::shared::event_entry::EventEntry;
use crate::shared::file_handler::FileHandler;

mod tests;
pub mod executables_handler;

/// Opens the Windows based on the content of the Snapshot
pub fn open_window() -> std::io::Result<()> {
    let snapshot_handler: FileHandler = FileHandler::new(shared::snapshot_path());
    let to_be_restored = snapshot_handler.read_file().expect("Could not read file");

    let mut executables_handler = ExecutablesHandler::new(shared::executables_path());
    executables_handler.init();

    for line in to_be_restored {
        let entry = EventEntry::new_without_event_type(&EventEntry::split(&line));
        match executables_handler.get_executable_entry(entry.class().trim()) {
            Some(executable_entry) => open_window_command(entry.workspace(), &executable_entry),
            None => open_window_command(entry.workspace(), entry.class())
        }
    }
    fs::remove_file(shared::snapshot_path())?;
    Ok(())
}

fn open_window_command(workspace: &str, to_be_opened: &str) {
    let _command = Command::new("hyprctl")
        .args(["dispatch", "exec", format!("[workspace {} silent] {}", workspace, to_be_opened).as_str()])
        .output()
        .expect("Failed to run hyprctl");
}