use std::sync::OnceLock;
use regex::Regex;

/// The EventEntry handles everything related to the contents of the open window event
///
/// * `address` := The hyprland address
/// * `workspace` := The hyprland workspace
/// * `class` := The hyprland class
/// * `title` := The hyprland title
#[derive(Debug)]
pub struct EventEntry {
    address: String,
    workspace: String,
    class: String,
    title: String

}
impl EventEntry {
    fn delimiter() -> &'static Regex {
        static DELIMITER: OnceLock<Regex> = OnceLock::new();
        DELIMITER.get_or_init(|| {Regex::new(r",|>>").unwrap()})
    }

    /// Splits the given String into a Vector\<String> by using , and/or >> as delimiter
    ///
    /// # Arguments
    /// * `line` := String you want to split
    ///
    /// # Returns
    /// A Vector of split elements as Strings
     pub fn split(line: &String) -> Vec<String> {
        Self::delimiter()
            .split(line)
            .map(|s| s.trim().to_string())
            .collect()
    }

    /// Constructs a new EventEntry with the Event type
    ///
    /// # Arguments
    /// * `event` := A Vector\<String> which represents the event
    ///     - `event[0]` := contains the Event type
    ///     - `event[1]` := contains the address
    ///     - `event[2]` := contains the workspace
    ///     - `event[3]` := contains the class
    ///     - `event[4]` := contains the title
    ///
    /// # Returns
    /// Self
    pub fn new(event: &Vec<String>) -> Self {
        let mut modified_address = event[1].clone();
        if !event[1].starts_with("0x") { modified_address = format!("0x{}", event[1]); }
        Self {
            address: modified_address,
            workspace: event[2].clone(),
            class: event[3].clone(),
            title: event[4].clone(),
        }
    }

    /// Constructs a new EventEntry without the Event type
    ///
    /// # Arguments
    /// * `event` := A Vector\<String> which represents the event
    ///     - `event[0]` := contains the address
    ///     - `event[1]` := contains the workspace
    ///     - `event[2]` := contains the class
    ///     - `event[3]` := contains the title
    ///
    /// # Returns
    /// Self
    pub fn new_without_event_type(event: &Vec<String>) -> Self {
        let mut modified_address = event[0].clone();
        if !event[0].starts_with("0x") { modified_address = format!("0x{}", event[0]); }
        Self {
            address: modified_address,
            workspace: event[1].clone(),
            class: event[2].clone(),
            title: event[3].clone(),
        }
    }

    /// Returns the content of the struct EventEntry, concatenated as String
    pub fn to_string(&self) -> String {
        format!("{},{},{},{}", self.address, self.workspace, self.class, self.title)
    }

    /// Returns the address as &str
    pub fn address(&self) -> &str { self.address.as_str() }

    /// Returns the class as &str
    pub fn class(&self) -> &str { self.class.as_str() }

    /// Returns the workspace as &str
    pub fn workspace(&self) -> &str { self.workspace.as_str() }

}