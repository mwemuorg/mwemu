/*
    INI ("profile") files written by the emulated process.

    WriteProfileString / WritePrivateProfileString store values here and the
    Get* calls read them back, so a program that persists a setting sees it
    again. Nothing touches the host: files start empty. File, section and key
    names compare case-insensitively, like kernel32.
*/

use lazy_static::lazy_static;
use std::sync::Mutex;

/// File the non-private profile APIs use.
pub const WIN_INI: &str = "win.ini";

struct Entry {
    file: String,
    section: String,
    key: String,
    value: String,
}

lazy_static! {
    static ref ENTRIES: Mutex<Vec<Entry>> = Mutex::new(Vec::new());
}

/// Bare file name, lowercased: `C:\Windows\win.ini` and `WIN.INI` are one file.
fn file_key(file: &str) -> String {
    file.rsplit(['\\', '/'])
        .next()
        .unwrap_or(file)
        .to_ascii_lowercase()
}

/// WritePrivateProfileString semantics: no key deletes the whole section, no
/// value deletes the key, otherwise the value is set.
pub fn write(file: &str, section: &str, key: Option<&str>, value: Option<&str>) {
    let file = file_key(file);
    let mut entries = ENTRIES.lock().unwrap();
    let same_section = |e: &Entry| e.file == file && e.section.eq_ignore_ascii_case(section);
    match (key, value) {
        (None, _) => entries.retain(|e| !same_section(e)),
        (Some(key), None) => {
            entries.retain(|e| !(same_section(e) && e.key.eq_ignore_ascii_case(key)))
        }
        (Some(key), Some(value)) => {
            match entries
                .iter_mut()
                .find(|e| same_section(e) && e.key.eq_ignore_ascii_case(key))
            {
                Some(entry) => entry.value = value.to_string(),
                None => entries.push(Entry {
                    file,
                    section: section.to_string(),
                    key: key.to_string(),
                    value: value.to_string(),
                }),
            }
        }
    }
}

/// Stored value of `file` / `section` / `key`.
pub fn read(file: &str, section: &str, key: &str) -> Option<String> {
    let file = file_key(file);
    let entries = ENTRIES.lock().unwrap();
    entries
        .iter()
        .find(|e| {
            e.file == file
                && e.section.eq_ignore_ascii_case(section)
                && e.key.eq_ignore_ascii_case(key)
        })
        .map(|e| e.value.clone())
}
