/*
    Window classes and windows created by the emulated process.

    Anti-debug code creates a decoy window named like a debugger and checks
    that FindWindow finds it: a plugin that hides debugger windows also hides
    the decoy. So RegisterClass*, CreateWindowEx*, DestroyWindow, FindWindow*
    and EnumWindows share this table instead of returning canned values.

    Class names and titles compare case-insensitively, like user32.
*/

use lazy_static::lazy_static;
use std::sync::Mutex;

/// First atom of the range user32 hands out to registered classes.
const FIRST_CLASS_ATOM: u16 = 0xc001;
const FIRST_HWND: u64 = 0x0001_0010;
const HWND_STEP: u64 = 0x10;

struct WindowClass {
    atom: u16,
    name: String,
}

struct Window {
    hwnd: u64,
    parent: u64,
    class: String,
    title: String,
}

lazy_static! {
    static ref CLASSES: Mutex<Vec<WindowClass>> = Mutex::new(Vec::new());
    static ref WINDOWS: Mutex<Vec<Window>> = Mutex::new(Vec::new());
}

/// Register a class name and return its atom (the existing one if already registered).
pub fn register_class(name: &str) -> u16 {
    let mut classes = CLASSES.lock().unwrap();
    if let Some(class) = classes.iter().find(|c| c.name.eq_ignore_ascii_case(name)) {
        return class.atom;
    }
    let atom = classes
        .last()
        .map(|c| c.atom + 1)
        .unwrap_or(FIRST_CLASS_ATOM);
    classes.push(WindowClass {
        atom,
        name: name.to_string(),
    });
    atom
}

/// Why a class could not be unregistered.
pub enum UnregisterError {
    NotRegistered,
    HasWindows,
}

/// Remove a registered class; it must exist and own no windows.
pub fn unregister_class(name: &str) -> Result<(), UnregisterError> {
    let in_use = WINDOWS
        .lock()
        .unwrap()
        .iter()
        .any(|w| w.class.eq_ignore_ascii_case(name));
    let mut classes = CLASSES.lock().unwrap();
    let idx = classes
        .iter()
        .position(|c| c.name.eq_ignore_ascii_case(name))
        .ok_or(UnregisterError::NotRegistered)?;
    if in_use {
        return Err(UnregisterError::HasWindows);
    }
    classes.remove(idx);
    Ok(())
}

/// Class name registered under `atom`.
pub fn class_name(atom: u16) -> Option<String> {
    let classes = CLASSES.lock().unwrap();
    classes
        .iter()
        .find(|c| c.atom == atom)
        .map(|c| c.name.clone())
}

/// Record a new window and return its HWND.
pub fn create_window(parent: u64, class: &str, title: &str) -> u64 {
    let mut windows = WINDOWS.lock().unwrap();
    let hwnd = windows
        .last()
        .map(|w| w.hwnd + HWND_STEP)
        .unwrap_or(FIRST_HWND);
    windows.push(Window {
        hwnd,
        parent,
        class: class.to_string(),
        title: title.to_string(),
    });
    hwnd
}

/// Forget a window; false when the HWND is unknown.
pub fn destroy_window(hwnd: u64) -> bool {
    let mut windows = WINDOWS.lock().unwrap();
    match windows.iter().position(|w| w.hwnd == hwnd) {
        Some(idx) => {
            windows.remove(idx);
            true
        }
        None => false,
    }
}

/// FindWindowEx semantics: the first window with this `parent` that comes after
/// `after` (0 = from the start) and matches the class and title filters
/// (None = any). Returns 0 when nothing matches.
pub fn find_window(parent: u64, after: u64, class: Option<&str>, title: Option<&str>) -> u64 {
    let windows = WINDOWS.lock().unwrap();
    let start = if after == 0 {
        0
    } else {
        match windows.iter().position(|w| w.hwnd == after) {
            Some(idx) => idx + 1,
            None => return 0,
        }
    };
    windows[start..]
        .iter()
        .find(|w| {
            w.parent == parent
                && class.is_none_or(|c| w.class.eq_ignore_ascii_case(c))
                && title.is_none_or(|t| w.title.eq_ignore_ascii_case(t))
        })
        .map(|w| w.hwnd)
        .unwrap_or(0)
}

/// HWNDs of the top-level windows (no parent), in creation order.
pub fn top_level_windows() -> Vec<u64> {
    let windows = WINDOWS.lock().unwrap();
    windows
        .iter()
        .filter(|w| w.parent == 0)
        .map(|w| w.hwnd)
        .collect()
}
