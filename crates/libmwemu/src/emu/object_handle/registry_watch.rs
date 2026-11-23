use serde::{Deserialize, Serialize};

use super::{HandleManagement, HandleType};
use crate::windows::constants::errors::*;
use crate::windows::constants::security::{
    EVENT_MODIFY_STATE, GENERIC_ALL, GENERIC_READ, GENERIC_WRITE, KEY_NOTIFY, MAXIMUM_ALLOWED,
};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub(super) struct NameNotification {
    pub(super) event: u64,
    subtree: bool,
}

pub(super) fn descendant(candidate: &str, parent: &str) -> bool {
    candidate.as_bytes().get(parent.len()) == Some(&b'\\')
        && candidate
            .get(..parent.len())
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case(parent))
}

impl HandleManagement {
    pub(crate) fn watch_registry_names(&mut self, key: u64, event: u64, subtree: bool) -> u64 {
        let lookup = |handle: u64| self.handle_types.get(usize::try_from(handle).ok()?);
        let key = match lookup(key) {
            Some(HandleType::RegistryKey(entry)) if entry.path.is_empty() => {
                return STATUS_KEY_DELETED;
            }
            Some(HandleType::RegistryKey(entry))
                if entry.access.is_some_and(|access| {
                    access & (KEY_NOTIFY | GENERIC_READ | GENERIC_ALL | MAXIMUM_ALLOWED) == 0
                }) =>
            {
                return STATUS_ACCESS_DENIED;
            }
            Some(HandleType::RegistryKey(_)) => key as usize,
            Some(_) => return STATUS_OBJECT_TYPE_MISMATCH,
            None => return STATUS_INVALID_HANDLE,
        };
        let id = match lookup(event) {
            Some(HandleType::Waitable(id, grant)) => {
                if grant.access
                    & (EVENT_MODIFY_STATE | GENERIC_WRITE | GENERIC_ALL | MAXIMUM_ALLOWED)
                    == 0
                {
                    return STATUS_ACCESS_DENIED;
                }
                *id
            }
            Some(_) => return STATUS_OBJECT_TYPE_MISMATCH,
            None => return STATUS_INVALID_HANDLE,
        };
        if !self
            .waitables
            .get_mut(&id)
            .is_some_and(|object| object.set_event(false))
        {
            return STATUS_OBJECT_TYPE_MISMATCH;
        }
        let notice = NameNotification { event: id, subtree };
        let Some(HandleType::RegistryKey(entry)) = self.handle_types.get_mut(key) else {
            return STATUS_INVALID_HANDLE;
        };
        if !entry.notifications.contains(&notice) {
            entry.notifications.push(notice);
        }
        STATUS_PENDING
    }

    pub(crate) fn registry_names_changed(&mut self, parent: &str) {
        let mut ready = Vec::new();
        for (_, handle) in &mut self.handle_types {
            if let HandleType::RegistryKey(key) = handle {
                let direct = key.path.eq_ignore_ascii_case(parent);
                let child = descendant(parent, &key.path);
                ready.extend(
                    key.notifications
                        .extract_if(.., |watch| direct || (child && watch.subtree)),
                );
            }
        }
        self.finish_registry_notifications(ready);
    }

    pub(super) fn finish_registry_notifications(&mut self, notifications: Vec<NameNotification>) {
        for watch in notifications {
            if let Some(object) = self.waitables.get_mut(&watch.event) {
                object.set_event(true);
            }
            self.release_unreferenced_waitable(watch.event);
        }
    }

    pub(super) fn release_unreferenced_waitable(&mut self, id: u64) {
        let referenced = self.handle_types.iter().any(|(_, handle)| match handle {
            HandleType::Waitable(other, _) => *other == id,
            HandleType::RegistryKey(key) => key.notifications.iter().any(|watch| watch.event == id),
            _ => false,
        });
        if !referenced {
            self.waitables.remove(&id);
        }
    }
}
