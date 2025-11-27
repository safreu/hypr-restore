mod ignored_handler_tests;

use crate::shared::file_handler::FileHandler;
use std::collections::HashSet;
use std::path::PathBuf;

/// The IgnoredHandler evaluates given Elements and decides if they should be skipped
pub struct IgnoredHandler {
    ignored_classes: HashSet<String>,
}

impl IgnoredHandler {
    /// Constructs a new IgnoredHandler
    ///
    /// # Arguments
    /// * `path` := The path to the file where classes which should be skipped are stored
    ///
    /// # Returns
    /// Self
    pub fn new(path: PathBuf) -> Self {
        let reader = FileHandler::new(path);
        let ignored_classes = match reader.read_file() {
            Ok(file) => file,
            Err(_) => panic!("Failed to read the ignored Classes file"),
        };
        Self { ignored_classes }
    }

    /// Evaluates if an open window event should be skipped (not inserted to the DB)
    ///
    /// # Arguments
    /// * `class` := The class of an open window event
    /// * `address` := The address of an open window event
    ///
    /// # Returns
    /// True in case it should be skipped, else False
    pub fn should_ignore(&self, class: &str) -> bool {
        self.ignored_classes
            .iter()
            .any(|ignore| class.to_lowercase().eq(ignore))
    }
}
