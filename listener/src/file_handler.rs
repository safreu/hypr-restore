use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use crate::event_entry::EventEntry;

pub struct FileHandler {
    path: String,
}

impl FileHandler {
    pub fn new(path: String) -> Self {
        FileHandler { path }
    }

    pub fn write(&mut self, printable: &str) -> io::Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .expect("Could not open file");
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}", printable).expect("Failed to write to file");
        writer.flush()
    }

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

    pub fn read_file(&self) -> io::Result<HashSet<String>> {
        let file = File::open(&self.path).expect("Failed to open ignored classes");
        let reader = BufReader::new(file);
        let mut set:HashSet<String> = HashSet::new();

        for line in reader.lines() {
            set.insert(line?.to_lowercase());
        };
        Ok(set)
    }
}