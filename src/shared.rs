use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;

pub mod event_entry;
pub mod file_handler;
const DB_FILE: &str = "tracker.db";
const IGNORE_FILE: &str = "classes.ignore";
const SNAPSHOT_FILE: &str = "tracker.snapshot";
const EXECUTABLE_FILE: &str = "executables.path";
const HYPR_RESTORE_PATH: &str = ".local/share/hypr_restore";

/// Returns the path to the DB File based on the home directory
///
/// # Returns
/// PathBuf
pub fn db_path() -> PathBuf {
    dirs::home_dir()
        .expect("Cannot determine home directory")
        .join(HYPR_RESTORE_PATH)
        .join(DB_FILE)
}

/// Returns the path to the Ignore File based on the home directory
///
/// # Returns
/// PathBuf
pub fn ignore_path() -> PathBuf {
    dirs::home_dir()
        .expect("Cannot determine home directory")
        .join(HYPR_RESTORE_PATH)
        .join(IGNORE_FILE)
}

/// Returns the path to the Snapshot File based on the home directory
///
/// # Returns
/// PathBuf
pub fn snapshot_path() -> PathBuf {
    dirs::home_dir()
        .expect("Cannot determine home directory")
        .join(HYPR_RESTORE_PATH)
        .join(SNAPSHOT_FILE)
}

/// Returns the path to the Executables File based on the home directory
///
/// # Returns
/// PathBuf
pub fn executables_path() -> PathBuf {
    dirs::home_dir()
        .expect("Cannot determine home directory")
        .join(HYPR_RESTORE_PATH)
        .join(EXECUTABLE_FILE)
}

/// Gets the Pid of the window process based on the hyprland address
///
/// # Arguments
///
/// `address` := the hyprland address, can be obtained via the open window event
///
/// # Returns
/// The PID
pub fn get_pid(address: &str) -> i64 {
    if std::env::var("TEST_NO_HYPRCTL").is_ok() {
        return 0;
    }

    let pid: i64 = 0;
    let output = Command::new("hyprctl")
        .args(["clients", "-j"])
        .output()
        .expect("Failed to run hyprctl");

    if !output.status.success() {
        return 0;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);

    let json: Value = serde_json::from_str(&stdout).expect("Failed to parse output");

    if let Some(array) = json.as_array() {
        for value in array {
            if value["address"] == address {
                return value["pid"].as_i64().unwrap_or(0);
            }
        }
    }
    pid
}
