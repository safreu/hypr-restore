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

    pub fn new(address: String, workspace: String, class: String, title: String) -> Self {
        let mut modified_address = address.clone();
        if !address.starts_with("0x") { modified_address = format!("0x{}", address); }
        Self {
            address: modified_address,
            workspace,
            class,
            title
        }
    }
    
    pub fn to_string(&self) -> String {
        format!("{},{},{},{}", self.address, self.workspace, self.class, self.title)
    }

    pub fn address(&self) -> &str {
        self.address.as_str()
    }

}