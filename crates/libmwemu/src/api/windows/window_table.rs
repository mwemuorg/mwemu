use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

const FIRST_CLASS_ATOM: u16 = 0xc001;
const FIRST_HWND: u64 = 0x0001_0010;
const HWND_STEP: u64 = 0x10;

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Window {
    pub style: u32,
    pub extended_style: u32,
    pub instance: u64,
    pub user_data: u64,
    pub parent: u64,
    pub class: String,
    pub title: String,
    pub thread: u64,
    pub process: u32,
    pub control_id: u64,
    pub wide: bool,
    pub enabled: bool,
    pub bounds: [i32; 4],
    pub menu: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct PostedMessage {
    pub hwnd: u64,
    pub id: u32,
    pub wparam: u64,
    pub lparam: u64,
    pub time: u32,
    pub point: [i32; 2],
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub(crate) struct UserState {
    pub messages: BTreeMap<u64, Vec<PostedMessage>>,
    pub message_times: BTreeMap<u64, u32>,
    pub focus: BTreeMap<u64, u64>,
    pub foreground: u64,
    pub cursor: u64,
    pub cursor_position: Option<[i32; 2]>,
    pub cursor_count: BTreeMap<u64, i32>,
    pub dpi_aware: bool,
    pub gui_threads: std::collections::BTreeSet<u64>,
    pub menus: std::collections::BTreeSet<u64>,
    pub objects: BTreeMap<u64, (String, bool)>,
    pub last_object: u64,
}

impl UserState {
    pub fn allocate(&mut self) -> Option<u64> {
        const FIRST_USER_OBJECT: u64 = 0x4000_0000;
        self.last_object = self.last_object.max(FIRST_USER_OBJECT).checked_add(1)?;
        Some(self.last_object)
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct WindowClass {
    name: String,
    wide: bool,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub(crate) struct WindowTable {
    classes: BTreeMap<u16, WindowClass>,
    windows: BTreeMap<u64, Window>,
    last_window: u64,
    pub user: UserState,
}

pub(crate) enum UnregisterError {
    NotRegistered,
    HasWindows,
}

impl WindowTable {
    pub fn contains(&self, hwnd: u64) -> bool {
        self.windows.contains_key(&hwnd)
    }
    pub fn get(&self, hwnd: u64) -> Option<&Window> {
        self.windows.get(&hwnd)
    }
    pub fn get_mut(&mut self, hwnd: u64) -> Option<&mut Window> {
        self.windows.get_mut(&hwnd)
    }

    pub fn register_class(&mut self, name: &str, wide: bool) -> Option<u16> {
        if self
            .classes
            .values()
            .any(|s| s.name.eq_ignore_ascii_case(name))
        {
            return None;
        }
        let atom = (FIRST_CLASS_ATOM..=u16::MAX).find(|key| !self.classes.contains_key(key))?;
        self.classes.insert(
            atom,
            WindowClass {
                name: name.to_owned(),
                wide,
            },
        );
        Some(atom)
    }

    pub fn unregister_class(&mut self, name: &str) -> Result<(), UnregisterError> {
        let atom = self
            .classes
            .iter()
            .find(|(_, s)| s.name.eq_ignore_ascii_case(name))
            .map(|(atom, _)| *atom)
            .ok_or(UnregisterError::NotRegistered)?;
        if self
            .windows
            .values()
            .any(|w| w.class.eq_ignore_ascii_case(name))
        {
            return Err(UnregisterError::HasWindows);
        }
        self.classes.remove(&atom);
        Ok(())
    }

    pub fn class_name(&self, atom: u16) -> Option<&str> {
        self.classes.get(&atom).map(|class| class.name.as_str())
    }

    pub fn class_wide(&self, name: &str) -> Option<bool> {
        self.classes
            .values()
            .find(|class| class.name.eq_ignore_ascii_case(name))
            .map(|class| class.wide)
    }

    pub fn insert(&mut self, window: Window) -> Option<u64> {
        let hwnd = if self.last_window == 0 {
            FIRST_HWND
        } else {
            self.last_window.checked_add(HWND_STEP)?
        };
        self.user.gui_threads.insert(window.thread);
        self.windows.insert(hwnd, window);
        self.last_window = hwnd;
        Some(hwnd)
    }

    pub fn is_child(&self, ancestor: u64, mut hwnd: u64) -> bool {
        if ancestor == 0 {
            return false;
        }
        while let Some(window) = self.get(hwnd) {
            hwnd = window.parent;
            if hwnd == ancestor {
                return true;
            }
        }
        false
    }

    pub fn control(&self, parent: u64, id: u32) -> u64 {
        if !self.contains(parent) {
            return 0;
        }
        self.windows
            .iter()
            .find(|(_, w)| w.parent == parent && w.control_id as u32 == id)
            .map_or(0, |(&hwnd, _)| hwnd)
    }

    pub fn enumerate(&self, parent: u64) -> Vec<u64> {
        if parent == 0 {
            return self
                .windows
                .iter()
                .filter_map(|(&h, w)| (w.parent == 0).then_some(h))
                .collect();
        }
        let mut list = Vec::new();
        let mut parents = vec![parent];
        while let Some(next) = parents.pop() {
            for (&h, _) in self.windows.iter().filter(|(_, w)| w.parent == next) {
                list.push(h);
                parents.push(h);
            }
        }
        list
    }

    pub fn destroy(&mut self, hwnd: u64) -> bool {
        if !self.contains(hwnd) {
            return false;
        }
        for child in self.enumerate(hwnd) {
            self.windows.remove(&child);
        }
        self.windows.remove(&hwnd);
        true
    }

    pub fn find(&self, parent: u64, after: u64, class: Option<&str>, title: Option<&str>) -> u64 {
        if after != 0 && self.get(after).is_none_or(|w| w.parent != parent) {
            return 0;
        }
        self.windows
            .iter()
            .find(|(h, w)| {
                **h > after
                    && w.parent == parent
                    && class.is_none_or(|s| w.class.eq_ignore_ascii_case(s))
                    && title.is_none_or(|s| w.title.eq_ignore_ascii_case(s))
            })
            .map_or(0, |(&h, _)| h)
    }
}
