use std::{fs, io};
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;
use serde_json::Value;

pub mod event_entry;
pub mod file_handler;
pub const DB_FILE: &str = "tracker.db";
pub const IGNORE_FILE: &str = "classes.ignore";
pub const SNAPSHOT_FILE: &str = "tracker.snapshot";
pub const EXECUTABLE_FILE: &str = "executables.path";

pub fn db_path() -> PathBuf {
    dirs::home_dir()
        .expect("Cannot determine home directory")
        .join(".local/share/hypr_restore")
        .join(DB_FILE)
}

pub fn ignore_path() -> PathBuf {
    dirs::home_dir()
        .expect("Cannot determine home directory")
        .join(".local/share/hypr_restore")
        .join(IGNORE_FILE)
}

pub fn snapshot_path() -> PathBuf {
    dirs::home_dir()
        .expect("Cannot determine home directory")
        .join(".local/share/hypr_restore")
        .join(SNAPSHOT_FILE)
}

pub fn executables_path() -> PathBuf {
    dirs::home_dir()
        .expect("Cannot determine home directory")
        .join(".local/share/hypr_restore")
        .join(EXECUTABLE_FILE)
}

pub fn get_pid(address: &str) -> i64 {
    let pid: i64= 0;
    let output = Command::new("hyprctl")
        .args(["clients", "-j"])
        .output()
        .expect("Failed to run hyprctl");

    if !output.status.success() { return 0; }
    let stdout = String::from_utf8_lossy(&output.stdout);

    let json: Value = serde_json::from_str(&stdout)
        .expect("Failed to parse output");

    if let Some(array) = json.as_array() {
        for value in array {
            if value["address"] == address {
                return value["pid"].as_i64().unwrap_or(0);
            }
        }
    }
    pid
}

/// Searches {pid}/environ for the specified flag and returns its value
///
/// # Arguments
///
/// * `flag_to_search` := the Flag you want the value for must be the exact Name and must contain = at the end
/// * `pid` := the pid you want to search for the specified flag
///
/// # Returns
/// The String containing the value or an io Error
pub fn get_env_value(flag_to_search: &str, pid: &i64) -> Result<String, io::Error> {
    let path = format!("/proc/{pid}/environ");
    let content = fs::read_to_string(&path)?;

    for entry in content.split('\0') {
        if entry.starts_with(flag_to_search) {
            if let Some((_key, value)) = entry.split_once('=') {
                return Ok(value.to_string());
            }
        }
    }
    Err(io::Error::new(ErrorKind::NotFound, "Flag not found"))
}
