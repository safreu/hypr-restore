use std::collections::HashSet;
use crate::file_handler::FileHandler;

pub struct IgnoredClasses {
    ignored_classes: HashSet<String>,
}

impl IgnoredClasses {
    pub fn new(path: &str) -> Self {
        let reader = FileHandler::new(path.to_string());
        let ignored_classes = match reader.read_file() {
            Ok(file) => file,
            Err(_) => panic!("Failed to read file"),
        };

        Self { ignored_classes }
    }

    pub fn should_ignore(&self, class: &str) -> bool {
        self.ignored_classes.iter().any(|ignore| class.to_lowercase().contains(ignore))
    }
}

