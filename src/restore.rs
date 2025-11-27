use crate::restore::executables_handler::ExecutablesHandler;
use crate::shared::event_entry::EventEntry;
use crate::shared::file_handler::FileHandler;
use crate::shared::{self};
use std::fs::{self};
use std::process::Command;

pub mod executables_handler;
mod restore_tests;

/// Opens the Windows based on the content of the Snapshot
pub fn open_window() -> std::io::Result<()> {
    let snapshot_handler: FileHandler = FileHandler::new(shared::snapshot_path());
    let to_be_restored = snapshot_handler
        .read_file_as_vec()
        .expect("Could not read file");

    let mut to_be_restored_as_events: Vec<EventEntry> = to_be_restored
        .iter()
        .map(|entry| EventEntry::new_without_event_type(&EventEntry::split(entry)))
        .collect();

    let batch_arg = process_events_to_exec_batch_arg(&to_be_restored_as_events);
    let _ = Command::new("hyprctl")
        .args(["--batch", &batch_arg])
        .output()
        .expect("Failed to run Command");

    let batch_arg = process_events_to_move_batch_arg(&mut to_be_restored_as_events);
    let _ = Command::new("hyprctl")
        .args(["--batch", &batch_arg])
        .output()
        .expect("Failed to run Command");

    fs::remove_file(shared::snapshot_path())?;
    Ok(())
}

fn process_events_to_exec_batch_arg(event_entries: &Vec<EventEntry>) -> String {
    let mut executables_handler = ExecutablesHandler::new(shared::executables_path());
    executables_handler.init();

    let mut batch_command: Vec<String> = Vec::new();

    for entry in event_entries {
        let to_be_opened = match executables_handler.get_executable_entry(entry.class().trim()) {
            Some(executables_entry) => executables_entry,
            None => entry.class().to_string(),
        };

        let cmd = format!(
            "dispatch exec [workspace {} silent] {}",
            entry.workspace(),
            to_be_opened
        );

        batch_command.push(cmd);
    }

    batch_command.join(" ; ")
}

fn process_events_to_move_batch_arg(executed_event_entries: &mut Vec<EventEntry>) -> String {
    let mut batch_command: Vec<String> = Vec::new();

    let db_handler = FileHandler::new(shared::db_path());
    let mut restored: Vec<String> = vec![];

    for i in 1..11 {
        restored = db_handler.read_file_as_vec().expect("Failed to read file");
        if restored.len() >= executed_event_entries.len() {
            break;
        }
        if i == 11 {
            panic!("Failed to get all restore events")
        }
    }

    for line in restored {
        let event_entry = EventEntry::new_without_event_type(&EventEntry::split(&line));

        let same_class_indices: Vec<usize> = executed_event_entries
            .iter()
            .enumerate()
            .filter(|(_, e)| e.class() == event_entry.class())
            .map(|(i, _)| i)
            .collect();

        if same_class_indices.is_empty() {
            continue;
        }

        let matching_workspace_indices: Option<usize> = same_class_indices
            .iter()
            .find(|&&i| executed_event_entries[i].workspace() == event_entry.workspace())
            .copied();

        if let Some(index) = matching_workspace_indices {
            executed_event_entries.remove(index);
            continue;
        }

        let first_index = same_class_indices[0];

        let cmd = format!(
            "dispatch movetoworkspacesilent {},address:{}",
            executed_event_entries[first_index].workspace(),
            event_entry.address()
        );

        batch_command.push(cmd);

        executed_event_entries.remove(first_index);
    }
    batch_command.join(" ; ")
}
