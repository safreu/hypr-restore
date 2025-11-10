use std::collections::HashSet;
use std::fs;
use crate::file_handler::FileHandler;
use utils::utils;

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

     fn should_skip(address: &str) -> bool {
        let pid = utils::get_pid(address);
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