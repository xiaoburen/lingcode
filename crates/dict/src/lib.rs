// This file contains the main implementation for the dictionary library, which handles dictionary-related functionalities. 

pub mod format;

pub struct Dictionary {
    entries: Vec<String>,
}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: String) {
        self.entries.push(entry);
    }

    pub fn get_entries(&self) -> &Vec<String> {
        &self.entries
    }
}