use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{PathBuf};
use log::info;
use crate::event_entry::EventEntry;

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
            .expect("Could not open file");
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}", printable).expect("Failed to write to file");
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
        let file = File::create(&self.path)?;
        let mut writer = BufWriter::new(file);

        for entry in entries {
            writeln!(writer, "{},{}", entry.0, entry.1).expect("Failed to write to file");
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
    pub fn remove_line(&mut self, table: &mut Vec<EventEntry>) -> io::Result<()>  {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)?;
        let mut writer = BufWriter::new(file);

        for line in table {
            writeln!(writer, "{}", line.to_string())?;
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
                File::create(&self.path)?;
                return Ok(HashSet::new());
            },
        };
        let reader = BufReader::new(file);
        let mut set:HashSet<String> = HashSet::new();

        for line in reader.lines() {
            set.insert(line?.to_lowercase());
        };
        Ok(set)
    }
}