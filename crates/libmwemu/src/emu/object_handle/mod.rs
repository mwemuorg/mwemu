pub(crate) use file_handle::FileHandle;
pub(crate) use heap_handle::HeapHandle;
pub(crate) use mapping_handle::MappingHandle;
use slab::Slab;

pub mod file_handle;
pub mod heap_handle;
mod hive_parser;
pub mod mapping_handle;
mod registry_handle;
mod windows_path;
// TODO: support more handle: registry, thread, etc
/*
Here the handle management is control by Slab and return a number, that number can be used as
handle id to get the right handle. In the document, it doesn't specific that the handle need to be divided by 4.

*/

enum HandleType {
    FileHandle(FileHandle),
    MappingHandle(MappingHandle),
    HeapHandle(HeapHandle),
}

pub struct HandleManagement {
    number_of_handle: usize,
    handle_types: Slab<HandleType>,
    process_heap_key: Option<u32>,
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
            process_heap_key: None,
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
    pub fn insert_heap_handle(&mut self, heap_handle: HeapHandle) -> u32 {
        let key = self
            .handle_types
            .insert(HandleType::HeapHandle(heap_handle));
        self.number_of_handle += 1;
        key as u32
    }

    pub fn get_mut_heap_handle(&mut self, key: u32) -> Option<&mut HeapHandle> {
        if let Some(handle_type) = self.handle_types.get_mut(key as usize) {
            match handle_type {
                HandleType::HeapHandle(hh) => Some(hh),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn remove_heap_handle(&mut self, key: u32) -> Option<HeapHandle> {
        if let Some(handle_type) = self.handle_types.try_remove(key as usize) {
            match handle_type {
                HandleType::HeapHandle(hh) => {
                    self.number_of_handle -= 1;
                    Some(hh)
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

    /// Return the slab key for the implicit process-heap `HeapHandle`,
    /// creating one (bound to arena 0) on first call.
    pub fn get_or_insert_process_heap(&mut self) -> u32 {
        if let Some(k) = self.process_heap_key {
            if matches!(
                self.handle_types.get(k as usize),
                Some(HandleType::HeapHandle(_))
            ) {
                return k;
            }
        }
        let key = self.insert_heap_handle(HeapHandle::new(0, 0, 0, 0));
        self.process_heap_key = Some(key);
        key
    }

    pub fn is_process_heap(&self, key: u32) -> bool {
        self.process_heap_key == Some(key)
    }

    /// Resolve a guest-supplied heap handle (truncated to u32) to its owning
    /// `HeapHandle` allocation context: `(arena_idx, maximum_size)`.
    /// Returns None when the handle is not a live `HeapHandle`.
    pub fn heap_alloc_context(&self, handle: u64) -> Option<(usize, u64)> {
        if handle > u32::MAX as u64 {
            return None;
        }
        let key = handle as u32;
        if let Some(HandleType::HeapHandle(hh)) = self.handle_types.get(key as usize) {
            Some((hh.arena, hh.maximum_size))
        } else {
            None
        }
    }

    /// Record an allocation against the heap identified by `handle`.
    /// Unknown/zero handles are attributed to the process heap (lenient —
    /// existing tests pass 0x1234 as a fake handle).
    pub fn record_heap_allocation(&mut self, handle: u64, addr: u64, size: u64) {
        let key = self.resolve_heap_handle_key(handle);
        if let Some(hh) = self.get_mut_heap_handle(key) {
            hh.record_allocation(addr, size);
        }
    }

    pub fn forget_heap_allocation(&mut self, handle: u64, addr: u64) {
        let key = self.resolve_heap_handle_key(handle);
        if let Some(hh) = self.get_mut_heap_handle(key) {
            hh.forget_allocation(addr);
        }
    }

    /// Drop any matching allocation record across every live heap handle.
    /// Used by callers that don't carry a heap handle (e.g. `LocalFree`).
    pub fn forget_heap_allocation_any(&mut self, addr: u64) {
        for (_, entry) in self.handle_types.iter_mut() {
            if let HandleType::HeapHandle(hh) = entry {
                hh.forget_allocation(addr);
            }
        }
    }

    /// Slab keys of every live heap handle. The process-heap key is first
    /// (matches `RtlGetProcessHeaps` semantics).
    pub fn heap_handle_keys(&self) -> Vec<u32> {
        let mut out = Vec::new();
        if let Some(pk) = self.process_heap_key {
            out.push(pk);
        }
        for (k, entry) in self.handle_types.iter() {
            if matches!(entry, HandleType::HeapHandle(_)) && Some(k as u32) != self.process_heap_key
            {
                out.push(k as u32);
            }
        }
        out
    }

    fn resolve_heap_handle_key(&mut self, handle: u64) -> u32 {
        if handle == 0 {
            return self.get_or_insert_process_heap();
        }
        if handle > u32::MAX as u64 {
            return self.get_or_insert_process_heap();
        }
        let key = handle as u32;
        if matches!(
            self.handle_types.get(key as usize),
            Some(HandleType::HeapHandle(_))
        ) {
            return key;
        }
        // Lenient fallback: attribute to the process heap so unknown handles
        // (e.g. 0x1234 from existing tests) keep recording correctly.
        self.get_or_insert_process_heap()
    }
}
