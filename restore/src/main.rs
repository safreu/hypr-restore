use std::process::Command;
use listener::event_entry::EventEntry;
use listener::file_handler::FileHandler;
use shared::{DB_PATH, RESTORE_PATH};
use utils;
use snapshot::create_snapshot;

fn main() {
    open_window();
    std::thread::sleep(std::time::Duration::from_secs(2));
    move_window();
}

fn open_window() {
    let snapshot_handler: FileHandler = FileHandler::new(shared::SNAPSHOT_PATH.to_string());
    let to_be_restored = snapshot_handler.read_file().expect("Could not read file");

    for line in to_be_restored {
        let entry = EventEntry::new_without_event_type(&EventEntry::split(&line));
        println!("{}", entry.address());
        let pid = utils::get_pid(entry.address());
        println!("pid: {}", pid);
        match utils::get_env_value("_=", &pid) {
            Ok(value) => open_window_command(&value),
            Err(_) => open_window_command(&entry.class())
        }
    }
}

fn move_window() {
    let result = create_snapshot(DB_PATH, RESTORE_PATH, false);
    let restore_handler = FileHandler::new(RESTORE_PATH.to_string());
    let to_be_moved = restore_handler.read_file().expect("Could not read file");
    for line in to_be_moved {
        let entry = EventEntry::new_without_event_type(&EventEntry::split(&line));
        let pid = utils::get_pid(entry.address());
        if utils::get_env_value("IS_RESTORED=", &pid).expect("Could not read file") == "1" {
            move_window_command(entry.workspace(), entry.address());
        }
    }
}

fn open_window_command(to_be_opened: &str) {
    let _command = Command::new("hyprctl")
        .env("IS_RESTORED", "1")
        .args(["dispatch", "exec", to_be_opened])
        .output()
        .expect("Failed to run hyprctl");
}

fn move_window_command(workspace: &str, address: &str) {
    let _command = Command::new("hyprctl")
        .args(["dispatch", "movetoworkspacesilent", workspace, address])
        .output()
        .expect("Failed to run hyprctl");
}

