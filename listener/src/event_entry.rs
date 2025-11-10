use std::sync::OnceLock;
use regex::Regex;

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

     pub fn split(line: &String) -> Vec<String> {
        Self::delimiter()
            .split(line)
            .map(|s| s.trim().to_string())
            .collect()
    }

    pub fn new(event_as_string: &Vec<String>) -> Self {
        let mut modified_address = event_as_string[1].clone();
        if !event_as_string[1].starts_with("0x") { modified_address = format!("0x{}", event_as_string[1]); }
        Self {
            address: modified_address,
            workspace: event_as_string[2].clone(),
            class: event_as_string[3].clone(),
            title: event_as_string[4].clone(),
        }
    }

    pub fn new_without_event_type(event_as_string: &Vec<String>) -> Self {
        let mut modified_address = event_as_string[0].clone();
        if !event_as_string[0].starts_with("0x") { modified_address = format!("0x{}", event_as_string[0]); }
        Self {
            address: modified_address,
            workspace: event_as_string[1].clone(),
            class: event_as_string[2].clone(),
            title: event_as_string[3].clone(),
        }
    }
    
    pub fn to_string(&self) -> String {
        format!("{},{},{},{}", self.address, self.workspace, self.class, self.title)
    }

    pub fn address(&self) -> &str {
        self.address.as_str()
    }

    pub fn class(&self) -> &str { self.class.as_str() }

}