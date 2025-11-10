use std::process::Command;
use serde_json::Value;

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