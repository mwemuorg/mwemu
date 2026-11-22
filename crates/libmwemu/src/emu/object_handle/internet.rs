use super::{HandleManagement, HandleType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum InternetLibrary {
    WinInet,
    WinHttp,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) enum InternetObject {
    Session,
    Connection { server: String, port: u16 },
    Request { url: String },
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct InternetHandle {
    pub library: InternetLibrary,
    pub object: InternetObject,
    pub parent: Option<u64>,
}

impl HandleManagement {
    pub(crate) fn create_cache_group(&mut self) -> u64 {
        self.number_of_handle += 1;
        self.handle_types.insert(HandleType::UrlCacheGroup) as u64
    }

    pub(crate) fn delete_cache_group(&mut self, group: u64) -> bool {
        let Ok(key) = usize::try_from(group) else {
            return false;
        };
        if !matches!(self.handle_types.get(key), Some(HandleType::UrlCacheGroup)) {
            return false;
        }
        self.handle_types.remove(key);
        self.number_of_handle -= 1;
        true
    }

    pub(crate) fn open_internet(
        &mut self,
        library: InternetLibrary,
        object: InternetObject,
        parent: Option<u64>,
    ) -> u64 {
        self.number_of_handle += 1;
        self.handle_types
            .insert(HandleType::Internet(InternetHandle {
                library,
                object,
                parent,
            })) as u64
    }

    pub(crate) fn internet(&self, key: u64, library: InternetLibrary) -> Option<&InternetObject> {
        match self.handle_types.get(usize::try_from(key).ok()?)? {
            HandleType::Internet(handle) if handle.library == library => Some(&handle.object),
            _ => None,
        }
    }

    pub(crate) fn close_internet(&mut self, key: u64, library: InternetLibrary) -> bool {
        if self.internet(key, library).is_none() {
            return false;
        }
        let mut pending = vec![key];
        while let Some(parent) = pending.pop() {
            pending.extend(
                self.handle_types
                    .iter()
                    .filter_map(|(key, entry)| match entry {
                        HandleType::Internet(handle) if handle.parent == Some(parent) => {
                            Some(key as u64)
                        }
                        _ => None,
                    }),
            );
            self.handle_types.remove(parent as usize);
            self.number_of_handle -= 1;
        }
        true
    }
}
