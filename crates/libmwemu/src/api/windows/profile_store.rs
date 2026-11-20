//! INI files visible to one emulated process.
use serde::{Deserialize, Serialize};

pub const WIN_INI: &str = "win.ini";

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ProfileStore {
    entries: Vec<Entry>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Entry {
    file: String,
    section: String,
    key: String,
    value: String,
}

impl ProfileStore {
    pub fn read(&self, file: &str, section: &str, key: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|entry| entry.matches(file, section) && entry.key.eq_ignore_ascii_case(key))
            .map(|entry| entry.value.as_str())
    }

    pub fn write(&mut self, file: &str, section: &str, key: Option<&str>, value: Option<&str>) {
        match (key, value) {
            (None, _) => self.entries.retain(|entry| !entry.matches(file, section)),
            (Some(key), None) => self.entries.retain(|entry| {
                !(entry.matches(file, section) && entry.key.eq_ignore_ascii_case(key))
            }),
            (Some(key), Some(value)) => {
                if let Some(entry) = self.entries.iter_mut().find(|entry| {
                    entry.matches(file, section) && entry.key.eq_ignore_ascii_case(key)
                }) {
                    entry.value = value.to_string();
                } else {
                    self.entries.push(Entry {
                        file: file.into(),
                        section: section.into(),
                        key: key.into(),
                        value: value.into(),
                    });
                }
            }
        }
    }

    pub(crate) fn names(&self, file: &str, section: Option<&str>) -> Vec<&str> {
        let mut names: Vec<&str> = Vec::new();
        for entry in &self.entries {
            if !entry.file.eq_ignore_ascii_case(file) {
                continue;
            }
            let name = match section {
                Some(section) if entry.section.eq_ignore_ascii_case(section) => &entry.key,
                None => &entry.section,
                _ => continue,
            };
            if !names.iter().any(|other| other.eq_ignore_ascii_case(name)) {
                names.push(name);
            }
        }
        names
    }
}

impl Entry {
    fn matches(&self, file: &str, section: &str) -> bool {
        self.file.eq_ignore_ascii_case(file) && self.section.eq_ignore_ascii_case(section)
    }
}
