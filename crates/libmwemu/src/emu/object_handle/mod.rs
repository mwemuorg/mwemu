pub(crate) use file_handle::FileHandle;
pub(crate) use mapping_handle::MappingHandle;
use slab::Slab;

pub mod file_handle;
mod hive_parser;
pub mod mapping_handle;
mod registry_handle;
mod windows_path;
// TODO: support more handle: registry, thread, heap, etc
/*
Here the handle management is control by Slab and return a number, that number can be used as
handle id to get the right handle. In the document, it doesn't specific that the handle need to be divided by 4.

*/

enum HandleType {
    FileHandle(FileHandle),
    MappingHandle(MappingHandle),
}

pub struct HandleManagement {
    number_of_handle: usize,
    handle_types: Slab<HandleType>,
}

impl Default for HandleManagement {
    fn default() -> Self {
        Self::new()
    }
}

impl HandleManagement {
    pub fn new() -> Self {
        Self {
            handle_types: Slab::with_capacity(200),
            number_of_handle: 0,
        }
    }

    pub fn insert_file_handle(&mut self, file_handle: FileHandle) -> u32 {
        let key = self
            .handle_types
            .insert(HandleType::FileHandle(file_handle));
        self.number_of_handle += 1;
        key as u32 // Assuming u32 is sufficient for slab keys in your context
    }

    pub fn insert_mapping_handle(&mut self, mapping_handle: MappingHandle) -> u32 {
        let key = self
            .handle_types
            .insert(HandleType::MappingHandle(mapping_handle));
        self.number_of_handle += 1;
        key as u32
    }

    // Method to get a mutable reference to a FileHandle by its key
    pub fn get_mut_file_handle(&mut self, key: u32) -> Option<&mut FileHandle> {
        if let Some(handle_type) = self.handle_types.get_mut(key as usize) {
            match handle_type {
                HandleType::FileHandle(fh) => Some(fh),
                // Add other handle type matches if/when they are implemented
                _ => None, // Handle exists but is not a FileHandle
            }
        } else {
            None // Handle key does not exist
        }
    }

    pub fn get_mut_mapping_handle(&mut self, key: u32) -> Option<&mut MappingHandle> {
        if let Some(handle_type) = self.handle_types.get_mut(key as usize) {
            match handle_type {
                HandleType::MappingHandle(mh) => Some(mh),
                _ => None,
            }
        } else {
            None
        }
    }

    // Method to remove a FileHandle (useful for CloseHandle)
    pub fn remove_file_handle(&mut self, key: u32) -> Option<FileHandle> {
        let file_handle = self.get_mut_file_handle(key)?;
        file_handle.close();
        if let Some(handle_type) = self.handle_types.try_remove(key as usize) {
            match handle_type {
                HandleType::FileHandle(fh) => {
                    self.number_of_handle -= 1;
                    Some(fh)
                }
                _ => {
                    // Put it back if it wasn't a FileHandle, though this indicates a logic error
                    self.handle_types.insert(handle_type);
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn remove_mapping_handle(&mut self, key: u32) -> Option<MappingHandle> {
        if let Some(handle_type) = self.handle_types.try_remove(key as usize) {
            match handle_type {
                HandleType::MappingHandle(mh) => {
                    self.number_of_handle -= 1;
                    Some(mh)
                }
                _ => {
                    self.handle_types.insert(handle_type);
                    None
                }
            }
        } else {
            None
        }
    }
}
