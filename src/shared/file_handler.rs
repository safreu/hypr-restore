mod file_handler_tests;

use core::fmt;
use crate::shared::event_entry::EventEntry;
use log::info;
use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

/// The FileHandler handles writing and reading operations on a specified file
///
/// * `path` := is the path to the specified file
pub struct FileHandler {
    path: PathBuf,
}

impl FileHandler {
    /// Constructs a new FileHandler
    ///
    /// # Arguments
    /// * `path` := A PathBuf containing the path to the file you want to read/write
    ///
    /// # Returns
    /// Self
    pub fn new(path: PathBuf) -> Self {
        FileHandler { path }
    }

    /// Writes printable at the end of the file
    ///
    /// # Arguments
    /// * `printable` := A &str to write into the file
    pub fn write(&mut self, printable: &str) -> io::Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .unwrap_or_else(|e| self.panic_when_opening_file(e));

        let mut writer = BufWriter::new(file);

        writeln!(writer, "{}", printable).unwrap_or_else(|_| panic!("Failed to write (fn write) to file [{}]", self.path.display()));

        info!("Wrote {} into {}", printable, self.path.display());

        writer.flush()
    }

    /// Writes the content of entries to a file
    ///
    /// # Arguments
    /// * `entries` := A Hashmap<String, String> which gets written to a file
    ///
    /// # Returns
    /// An empty io::Result
    pub fn write_complete_hashmap(&mut self, entries: &HashMap<String, String>) -> io::Result<()> {
        let file = File::create(&self.path).unwrap_or_else(|e| self.panic_when_opening_file(e));

        let mut writer = BufWriter::new(file);

        for entry in entries {
            writeln!(writer, "{},{}", entry.0, entry.1).unwrap_or_else(|_| panic!("Failed to write (fn write_complete_hashmap) to [{}]", self.path.display()));
        }
        writer.flush()
    }

    /// Removes a line of a file by writing the content of table
    ///
    /// # Arguments
    /// * `table` := Is the content of the file without the line you want removed
    ///
    /// # Returns
    /// An empty io::Result
    pub fn remove_line(&mut self, table: &mut Vec<EventEntry>) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
            .unwrap_or_else(|e| self.panic_when_opening_file(e));

        let mut writer = BufWriter::new(file);

        for line in table {
            writeln!(writer, "{}", line).unwrap_or_else(|_| panic!("Failed to write (fn remove_line) to [{}]", self.path.display()));
        }
        writer.flush()
    }

    /// Reads the content of a file and writes it into a HashSet
    ///
    /// # Returns
    /// The content of the files as a HashSet wrapped inside io Result
    pub fn read_file(&self) -> io::Result<HashSet<String>> {
        let file = match File::open(&self.path) {
            Ok(file) => file,
            Err(_) => {
                File::create(&self.path).unwrap_or_else(|e| self.panic_when_opening_file(e));
                return Ok(HashSet::new());
            }
        };
        let reader = BufReader::new(file);
        let mut set: HashSet<String> = HashSet::new();

        for line in reader.lines() {
            set.insert(line?.to_lowercase());
        }
        Ok(set)
    }

    /// Reads the content of a file and writes it into a Vec
    ///
    /// # Returns
    /// The content of the files as a Vec wrapped inside io Result
    pub fn read_file_as_vec(&self) -> io::Result<Vec<String>> {
        let file = match File::open(&self.path) {
            Ok(file) => file,
            Err(_) => {
                File::create(&self.path).unwrap_or_else(|e| self.panic_when_opening_file(e));
                return Ok(Vec::new());
            }
        };

        let reader = BufReader::new(file);

        reader.lines().map(|line| Ok(line?.to_lowercase())).collect()
    }
    /// Returns the path as String
    pub fn get_path(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    /// just a function with a panic message to call if there is an error while opening/creating file
    fn panic_when_opening_file<T, E: fmt::Display>(&self, err: E) -> T {
        panic!("Failed to open file [{}]: {}", self.path.display(), err);
    }
}
