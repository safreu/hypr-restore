mod tests;
mod executables_handler;

use std::process::Command;
use shared::event_entry::EventEntry;
use shared::file_handler::FileHandler;
use shared::{EXECUTABLE_PATH, SNAPSHOT_PATH};
use crate::executables_handler::ExecutablesHandler;

fn main() {
    open_window();
}

fn open_window() {
    let snapshot_handler: FileHandler = FileHandler::new(SNAPSHOT_PATH.to_string());
    let to_be_restored = snapshot_handler.read_file().expect("Could not read file");

    let mut executables_handler = ExecutablesHandler::new(EXECUTABLE_PATH);
    executables_handler.init();

    for line in to_be_restored {
        let entry = EventEntry::new_without_event_type(&EventEntry::split(&line));
        match executables_handler.get_executable_entry(entry.class().trim()) {
            Some(executable_entry) => open_window_command(entry.workspace(), &executable_entry),
            None => open_window_command(entry.workspace(), entry.class())
        }
    }
}

fn open_window_command(workspace: &str, to_be_opened: &str) {
    println!("{}", to_be_opened);
    let _command = Command::new("hyprctl")
        .args(["dispatch", "exec", format!("[workspace {} silent] {}", workspace, to_be_opened).as_str()])
        .output()
        .expect("Failed to run hyprctl");
}