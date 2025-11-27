mod ignored_handler_tests;

use crate::shared::file_handler::FileHandler;
use log::info;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::{collections::HashSet, io};
/// The IgnoredHandler evaluates given Elements and decides if they should be skipped
pub struct IgnoredHandler {
    path: PathBuf,
    ignored_classes: HashSet<String>,
    content_hash: Vec<u8>,
}

impl IgnoredHandler {
    /// Constructs a new IgnoredHandler
    ///
    /// # Arguments
    /// * `path` := The path to the file where classes which should be skipped are stored
    ///
    /// # Returns
    /// Self
    pub fn new(path: PathBuf) -> io::Result<Self> {
        let ignored_classes = Self::load_file(&path);
        let content_hash = Self::compute_hash(&path)?;
        Ok(Self {
            path,
            ignored_classes,
            content_hash,
        })
    }

    fn load_file(path: &Path) -> HashSet<String> {
        let reader = FileHandler::new(path.to_path_buf());
        match reader.read_file() {
            Ok(file) => file,
            Err(_) => panic!("Failed to read the ignored Classes file"),
        }
    }

    fn compute_hash(path: &PathBuf) -> io::Result<Vec<u8>> {
        let data = fs::read(path)?;
        Ok(Sha256::digest(data).to_vec())
    }

    fn refresh_if_changed(&mut self) -> io::Result<()> {
        let hash = Self::compute_hash(&self.path)?;
        if self.content_hash != hash {
            self.content_hash = hash;
            self.ignored_classes = Self::load_file(&self.path);
            info!("File content of [{}] changed", self.path.display(),);
        }
        Ok(())
    }

    /// Evaluates if an open window event should be skipped (not inserted to the DB)
    ///
    /// # Arguments
    /// * `class` := The class of an open window event
    ///
    /// # Returns
    /// True in case it should be skipped, else False
    pub fn should_ignore(&mut self, class: &str) -> bool {
        self.refresh_if_changed().ok();

        let class = class.to_lowercase();

        self.ignored_classes.contains(&class)
    }
}
