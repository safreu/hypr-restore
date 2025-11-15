use std::collections::HashSet;
use std::path::PathBuf;
use shared::file_handler::FileHandler;
use shared;
pub struct IgnoredHandler {
    ignored_classes: HashSet<String>,
}

impl IgnoredHandler {
    pub fn new(path: PathBuf) -> Self {
        let reader = FileHandler::new(path);
        let ignored_classes = match reader.read_file() {
            Ok(file) => file,
            Err(_) => panic!("Failed to read file"),
        };
        Self { ignored_classes }
    }

    pub fn should_ignore(&self, class: &str, address: &str) -> bool {
        let ignore_flag = self.ignored_classes.iter().any(|ignore| class.to_lowercase().contains(ignore));
        let skip_flag = Self::should_skip(address);
        if ignore_flag || skip_flag { true }
        else { false }
    }

     fn should_skip(address: &str) -> bool {
         let pid = shared::get_pid(address);
         match shared::get_env_value("RESTORE_SKIP=", &pid) {
             Ok(value) => if value == "1" { true } else { false },
             Err(_) => false,
         }
     }
}