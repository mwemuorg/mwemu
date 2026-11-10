use crate::maps::mem64::Mem64;
use crate::maps::shared::{SharedStorage, SharedView};
use std::borrow::Cow;

impl Mem64 {
    pub fn bind_shared(&mut self, view: SharedView, storage: SharedStorage) {
        self.mem.clear();
        self.view = Some(view);
        self.shared = Some(storage);
    }

    pub fn shared_view(&self) -> Option<SharedView> {
        self.view
    }

    pub fn shared_tail(&self, address: u64) -> Option<(SharedView, SharedStorage)> {
        let mut view = self.view?;
        view.offset += (address - self.base_addr) as usize;
        Some((view, self.shared.as_ref()?.clone()))
    }

    pub fn detach_shared(&mut self) {
        if self.view.is_some() {
            self.mem = self.get_bytes().into_owned();
            self.view = None;
            self.shared = None;
        }
    }

    pub fn with_backing<T>(&self, read: impl FnOnce(&[u8]) -> T) -> T {
        if let (Some(view), Some(shared)) = (self.view, &self.shared) {
            let bytes = shared.bytes.borrow();
            read(&bytes[view.offset..view.offset + self.size()])
        } else {
            read(&self.mem)
        }
    }

    /// Inspect backing bytes without applying guest access permissions.
    pub fn get_bytes_range(&self, offset: usize, size: usize) -> Option<Cow<'_, [u8]>> {
        if offset.checked_add(size)? > self.size() {
            return None;
        }
        Some(self.backing_bytes(offset, size))
    }

    #[inline]
    pub fn backing_bytes(&self, offset: usize, size: usize) -> Cow<'_, [u8]> {
        if self.view.is_some() {
            Cow::Owned(self.with_backing(|bytes| bytes[offset..offset + size].to_vec()))
        } else {
            Cow::Borrowed(&self.mem[offset..offset + size])
        }
    }

    #[inline]
    pub fn write_backing(&mut self, offset: usize, bytes: &[u8]) {
        if let (Some(view), Some(shared)) = (self.view, &self.shared) {
            shared.bytes.borrow_mut()[view.offset + offset..view.offset + offset + bytes.len()]
                .copy_from_slice(bytes);
            shared.dirty.set(true);
        } else {
            self.mem[offset..offset + bytes.len()].copy_from_slice(bytes);
        }
    }
}
