use std::collections::HashSet;
use std::fs;
use std::process::Command;
use serde_json::Value;
use crate::file_handler::FileHandler;

pub struct IgnoredHandler {
    ignored_classes: HashSet<String>,
}

impl IgnoredHandler {
    pub fn new(path: &str) -> Self {
        let reader = FileHandler::new(path.to_string());
        let ignored_classes = match reader.read_file() {
            Ok(file) => file,
            Err(_) => panic!("Failed to read file"),
        };
        Self { ignored_classes }
    }

    pub fn should_ignore(&self, class: &str, address: &str) -> bool {
        let ignore_flag = self.ignored_classes.iter().any(|ignore| class.to_lowercase().contains(ignore));
        let skip_flag = Self::should_skip(format!("0x{}", address).as_str());
        if ignore_flag || skip_flag { true }
        else { false }
    }

    fn get_pid(address: &str) -> i64 {
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

     fn should_skip(address: &str) -> bool {
        let pid = Self::get_pid(address);
        let path = format!("/proc/{pid}/environ");
        match fs::read_to_string(&path) {
            Ok(content) => {
                for entry in content.split('\0') {
                    if !entry.is_empty() && entry.contains("RESTORE_SKIP=1") {
                        println!("{}", entry);
                        return true;
                    }
                }
                false
            }
            Err(_) => false
        }
    }
}