mod ignored_handler_tests;

use std::collections::HashSet;
use std::path::PathBuf;
use crate::shared;
use crate::shared::file_handler::FileHandler;

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
    pub fn should_ignore(&self, class: &str, address: &str) -> bool {
        let ignore_flag = self.ignored_classes.iter().any(|ignore| class.to_lowercase().eq(ignore));
        let skip_flag = Self::should_skip(address);
        if ignore_flag || skip_flag { true }
        else { false }
    }

    /// This Method evaluates if an application was started using the RESTORE_SKIP flag
    /// 
    /// # Arguments
    /// * `address` := The address of an open window event
    /// 
    /// # Returns
    /// True if the application was started with the RESTORE_SKIP flag, else False
     fn should_skip(address: &str) -> bool {
         let pid = shared::get_pid(address);
         match shared::get_env_value("RESTORE_SKIP=", &pid) {
             Ok(value) => if value == "1" { true } else { false },
             Err(_) => false,
         }
     }
}