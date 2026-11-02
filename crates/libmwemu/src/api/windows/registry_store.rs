/*
    Registry of the emulated process.

    Keys and values the process creates are stored here and read back by the
    Reg* APIs, so a program that persists state in the registry sees it again
    and a key that was never created does not exist. Nothing touches the host:
    the store starts with the predefined roots and a few standard keys.

    Paths are full, rooted at the predefined key names (`HKEY_CURRENT_USER\
    Software\...`). Key and value names compare case-insensitively, like the
    configuration manager. String values are stored as UTF-16, as on Windows.
*/

use lazy_static::lazy_static;
use std::sync::Mutex;

pub const REG_NONE: u32 = 0;
pub const REG_SZ: u32 = 1;
pub const REG_EXPAND_SZ: u32 = 2;
pub const REG_BINARY: u32 = 3;
pub const REG_DWORD: u32 = 4;
pub const REG_MULTI_SZ: u32 = 7;
pub const REG_QWORD: u32 = 11;

pub const HKEY_CLASSES_ROOT: u64 = 0x8000_0000;
pub const HKEY_CURRENT_USER: u64 = 0x8000_0001;
pub const HKEY_LOCAL_MACHINE: u64 = 0x8000_0002;
pub const HKEY_USERS: u64 = 0x8000_0003;
pub const HKEY_PERFORMANCE_DATA: u64 = 0x8000_0004;
pub const HKEY_CURRENT_CONFIG: u64 = 0x8000_0005;

/// Path a predefined handle stands for. HKCR and HKCC are views of HKLM keys.
pub fn predefined_path(hkey: u64) -> Option<&'static str> {
    // predefined handles are sign-extended 32-bit values on x64
    match hkey & 0xffff_ffff {
        HKEY_CLASSES_ROOT => Some("HKEY_LOCAL_MACHINE\\SOFTWARE\\Classes"),
        HKEY_CURRENT_USER => Some("HKEY_CURRENT_USER"),
        HKEY_LOCAL_MACHINE => Some("HKEY_LOCAL_MACHINE"),
        HKEY_USERS => Some("HKEY_USERS"),
        HKEY_PERFORMANCE_DATA => Some("HKEY_PERFORMANCE_DATA"),
        HKEY_CURRENT_CONFIG => {
            Some("HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Hardware Profiles\\Current")
        }
        _ => None,
    }
}

#[derive(Clone)]
pub struct Value {
    pub kind: u32,
    pub data: Vec<u8>,
}

struct Key {
    path: String,
    values: Vec<(String, Value)>,
}

/// Why a key could not be deleted.
pub enum DeleteError {
    NotFound,
    HasSubkeys,
}

/// Sizes RegQueryInfoKey reports; name lengths in characters.
#[derive(Default)]
pub struct KeyInfo {
    pub subkeys: u32,
    pub max_subkey_len: u32,
    pub values: u32,
    pub max_value_name_len: u32,
    pub max_value_len: u32,
}

lazy_static! {
    static ref KEYS: Mutex<Vec<Key>> = Mutex::new(seed());
}

const ROOTS: &[&str] = &[
    "HKEY_CURRENT_USER",
    "HKEY_LOCAL_MACHINE",
    "HKEY_USERS",
    "HKEY_PERFORMANCE_DATA",
];

/// Keys every Windows installation has.
const STANDARD_KEYS: &[&str] = &[
    "HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer",
    "HKEY_CURRENT_USER\\Software\\Classes",
    "HKEY_CURRENT_USER\\Control Panel\\International",
    "HKEY_CURRENT_USER\\Environment",
    "HKEY_LOCAL_MACHINE\\SOFTWARE\\Classes",
    "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion",
    "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
    "HKEY_LOCAL_MACHINE\\SOFTWARE\\WOW6432Node",
    "HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Control",
    "HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Services",
    "HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Hardware Profiles\\Current",
];

const NT_CURRENT_VERSION: &str =
    "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion";

fn seed() -> Vec<Key> {
    let mut keys: Vec<Key> = Vec::new();
    for root in ROOTS {
        keys.push(Key {
            path: root.to_string(),
            values: Vec::new(),
        });
    }
    for path in STANDARD_KEYS {
        insert_path(&mut keys, path);
    }
    let version = find_mut(&mut keys, NT_CURRENT_VERSION).unwrap();
    for (name, value) in [
        ("ProductName", sz("Windows 10 Pro")),
        ("EditionID", sz("Professional")),
        ("InstallationType", sz("Client")),
        ("CurrentVersion", sz("6.3")),
        ("CurrentBuild", sz("26100")),
        ("CurrentBuildNumber", sz("26100")),
        ("DisplayVersion", sz("24H2")),
        ("CurrentMajorVersionNumber", dword(10)),
        ("CurrentMinorVersionNumber", dword(0)),
        ("SystemRoot", sz("C:\\Windows")),
    ] {
        version.values.push((name.to_string(), value));
    }
    keys
}

/// REG_SZ value of `s`, NUL-terminated.
pub fn sz(s: &str) -> Value {
    Value {
        kind: REG_SZ,
        data: s
            .encode_utf16()
            .chain(std::iter::once(0))
            .flat_map(|c| c.to_le_bytes())
            .collect(),
    }
}

pub fn dword(v: u32) -> Value {
    Value {
        kind: REG_DWORD,
        data: v.to_le_bytes().to_vec(),
    }
}

/// `parent\sub` with separators normalized; an empty `sub` is `parent` itself.
pub fn join(parent: &str, sub: &str) -> String {
    let parts: Vec<&str> = sub.split('\\').filter(|p| !p.is_empty()).collect();
    if parts.is_empty() {
        parent.to_string()
    } else {
        format!("{}\\{}", parent, parts.join("\\"))
    }
}

fn parent_of(path: &str) -> Option<&str> {
    path.rfind('\\').map(|i| &path[..i])
}

fn find_mut<'a>(keys: &'a mut [Key], path: &str) -> Option<&'a mut Key> {
    keys.iter_mut().find(|k| k.path.eq_ignore_ascii_case(path))
}

fn find<'a>(keys: &'a [Key], path: &str) -> Option<&'a Key> {
    keys.iter().find(|k| k.path.eq_ignore_ascii_case(path))
}

/// Create `path` and its missing ancestors; true when `path` was new.
fn insert_path(keys: &mut Vec<Key>, path: &str) -> bool {
    if find(keys, path).is_some() {
        return false;
    }
    if let Some(parent) = parent_of(path) {
        insert_path(keys, parent);
    }
    keys.push(Key {
        path: path.to_string(),
        values: Vec::new(),
    });
    true
}

pub fn key_exists(path: &str) -> bool {
    find(&KEYS.lock().unwrap(), path).is_some()
}

/// Create the key (and missing parents) unless it exists; true when created.
/// None when the root is not a predefined key.
pub fn create_key(path: &str) -> Option<bool> {
    let mut keys = KEYS.lock().unwrap();
    let root = path.split('\\').next().unwrap_or("");
    find(&keys, root)?;
    Some(insert_path(&mut keys, path))
}

/// Delete a key without subkeys, or with `recursive` the key and its subtree.
pub fn delete_key(path: &str, recursive: bool) -> Result<(), DeleteError> {
    let mut keys = KEYS.lock().unwrap();
    find(&keys, path).ok_or(DeleteError::NotFound)?;
    let prefix = format!("{}\\", path.to_ascii_lowercase());
    let in_subtree = |k: &Key| k.path.to_ascii_lowercase().starts_with(&prefix);
    if !recursive && keys.iter().any(in_subtree) {
        return Err(DeleteError::HasSubkeys);
    }
    keys.retain(|k| !k.path.eq_ignore_ascii_case(path) && !in_subtree(k));
    Ok(())
}

/// Remove every value and subkey of `path`, keeping the key.
pub fn clear_key(path: &str) -> bool {
    let mut keys = KEYS.lock().unwrap();
    let prefix = format!("{}\\", path.to_ascii_lowercase());
    keys.retain(|k| !k.path.to_ascii_lowercase().starts_with(&prefix));
    match find_mut(&mut keys, path) {
        Some(key) => {
            key.values.clear();
            true
        }
        None => false,
    }
}

/// Set a value; false when the key does not exist. "" is the default value.
pub fn set_value(path: &str, name: &str, value: Value) -> bool {
    let mut keys = KEYS.lock().unwrap();
    let Some(key) = find_mut(&mut keys, path) else {
        return false;
    };
    match key
        .values
        .iter_mut()
        .find(|(n, _)| n.eq_ignore_ascii_case(name))
    {
        Some((_, v)) => *v = value,
        None => key.values.push((name.to_string(), value)),
    }
    true
}

pub fn get_value(path: &str, name: &str) -> Option<Value> {
    let keys = KEYS.lock().unwrap();
    find(&keys, path)?
        .values
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(name))
        .map(|(_, v)| v.clone())
}

/// Remove a value; false when the key or value does not exist.
pub fn delete_value(path: &str, name: &str) -> bool {
    let mut keys = KEYS.lock().unwrap();
    let Some(key) = find_mut(&mut keys, path) else {
        return false;
    };
    let before = key.values.len();
    key.values.retain(|(n, _)| !n.eq_ignore_ascii_case(name));
    key.values.len() != before
}

/// Names of the direct subkeys of `path`, in creation order.
pub fn subkeys(path: &str) -> Vec<String> {
    let keys = KEYS.lock().unwrap();
    keys.iter()
        .filter(|k| parent_of(&k.path).is_some_and(|p| p.eq_ignore_ascii_case(path)))
        .map(|k| k.path.rsplit('\\').next().unwrap_or("").to_string())
        .collect()
}

/// The `index`-th value of `path`, in creation order.
pub fn value_at(path: &str, index: usize) -> Option<(String, Value)> {
    let keys = KEYS.lock().unwrap();
    find(&keys, path)?.values.get(index).cloned()
}

pub fn key_info(path: &str) -> Option<KeyInfo> {
    let subkeys = subkeys(path);
    let keys = KEYS.lock().unwrap();
    let key = find(&keys, path)?;
    Some(KeyInfo {
        subkeys: subkeys.len() as u32,
        max_subkey_len: subkeys
            .iter()
            .map(|s| s.encode_utf16().count() as u32)
            .max()
            .unwrap_or(0),
        values: key.values.len() as u32,
        max_value_name_len: key
            .values
            .iter()
            .map(|(n, _)| n.encode_utf16().count() as u32)
            .max()
            .unwrap_or(0),
        max_value_len: key
            .values
            .iter()
            .map(|(_, v)| v.data.len() as u32)
            .max()
            .unwrap_or(0),
    })
}
