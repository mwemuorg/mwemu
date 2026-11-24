//! VFS nodes retain their logical length. Shared views cover whole guest pages.
use std::collections::BTreeSet;

use crate::maps::Maps;
use crate::maps::virtual_memory::PAGE_SIZE;
use crate::syscall::linux::constants::errors::{EACCES, EBADF, EFBIG, EISDIR, ENOMEM};

use super::{LinuxFiles, MAX_STORAGE, Node, O_ACCMODE, O_RDWR, O_WRONLY};

impl Node {
    fn mapping_parts(&self, offset: usize, count: usize) -> (&[u8], &[u8]) {
        let bytes = self.bytes.get(offset..).unwrap_or_default();
        let data = &bytes[..count.min(bytes.len())];
        let tail = self
            .page_tail
            .get(offset.saturating_sub(self.bytes.len())..)
            .unwrap_or_default();
        (data, &tail[..(count - data.len()).min(tail.len())])
    }
}

impl LinuxFiles {
    pub(crate) fn private_mapping_bytes(
        &self,
        fd: u32,
        offset: u64,
        size: usize,
    ) -> Result<Vec<u8>, u64> {
        let file = self.open.get(&fd).ok_or(EBADF)?;
        if file.flags & O_ACCMODE == O_WRONLY {
            return Err(EACCES);
        }
        let node = self.nodes.get(&file.inode).ok_or(EBADF)?;
        if node.directory() {
            return Err(EISDIR);
        }
        let offset = usize::try_from(offset).map_err(|_| EFBIG)?;
        let (bytes, tail) = node.mapping_parts(offset, size);
        Ok([bytes, tail].concat())
    }

    pub(crate) fn mapping_store(
        &mut self,
        maps: &mut Maps,
        fd: u32,
        end: u64,
        write: bool,
    ) -> Result<u64, u64> {
        let file = self.open.get(&fd).ok_or(EBADF)?;
        if file.flags & O_ACCMODE == O_WRONLY || (write && file.flags & O_ACCMODE != O_RDWR) {
            return Err(EACCES);
        }
        let inode = file.inode;
        let node = self.nodes.get(&inode).ok_or(EBADF)?;
        if node.directory() {
            return Err(EISDIR);
        }
        let size = usize::try_from(end).map_err(|_| EFBIG)?;
        let previous = self
            .mapping_stores
            .get(&inode)
            .and_then(|id| maps.shared_stores.get(id))
            .map_or(0, |store| store.len());
        let total: usize = self
            .mapping_stores
            .values()
            .filter_map(|id| maps.shared_stores.get(id))
            .map(|store| store.len())
            .sum();
        if size.saturating_sub(previous) > MAX_STORAGE.saturating_sub(total) {
            return Err(ENOMEM);
        }
        let store = *self
            .mapping_stores
            .entry(inode)
            .or_insert_with(|| maps.create_shared_storage(Vec::new()));
        let backing = maps.shared_stores.get(&store).ok_or(ENOMEM)?;
        let mut bytes = backing.bytes.borrow_mut();
        if bytes.len() < size {
            let old = bytes.len();
            bytes.resize(size, 0);
            let (data, tail) = node.mapping_parts(old, size - old);
            let split = old + data.len();
            bytes[old..split].copy_from_slice(data);
            bytes[split..split + tail.len()].copy_from_slice(tail);
        }
        Ok(store)
    }

    // Guest writes dirty the shared pages; descriptor writes dirty the logical file.
    // Publish each side at the syscall boundary while preserving the final page's padding.
    pub(crate) fn sync_mappings(&mut self, maps: &Maps) {
        for (inode, id) in &self.mapping_stores {
            let (Some(node), Some(store)) = (self.nodes.get_mut(inode), maps.shared_stores.get(id))
            else {
                continue;
            };
            let n = node.bytes.len().min(store.len());
            if store.file_dirty.replace(false) {
                let bytes = store.bytes.borrow();
                node.bytes[..n].copy_from_slice(&bytes[..n]);
                // tmpfs keeps writes after EOF in the final partial page for subsequent mmap.
                // read(2) and the length reported by stat still end at the logical file size.
                if n == node.bytes.len() {
                    let end = n.next_multiple_of(PAGE_SIZE as usize).min(bytes.len());
                    let tail = &bytes[n..end];
                    node.page_tail.clear();
                    if let Some(last) = tail.iter().rposition(|&byte| byte != 0) {
                        node.page_tail.extend_from_slice(&tail[..=last]);
                    }
                }
            }
            if std::mem::take(&mut node.mapping_dirty) {
                let mut bytes = store.bytes.borrow_mut();
                bytes[..n].copy_from_slice(&node.bytes[..n]);
                let tail_len = (bytes.len() - n).min(node.page_tail.len());
                bytes[n..n + tail_len].copy_from_slice(&node.page_tail[..tail_len]);
                bytes[n + tail_len..].fill(0);
            }
        }
    }

    pub(crate) fn restore_mappings(&mut self, maps: &Maps) {
        // Native snapshots save page contents; the transient dirty bit starts clear on restore.
        for id in self.mapping_stores.values() {
            if let Some(store) = maps.shared_stores.get(id) {
                store.file_dirty.set(true);
            }
        }
        self.sync_mappings(maps);
    }

    pub(crate) fn reap_mappings(&mut self, maps: &mut Maps) {
        if self.mapping_stores.is_empty() {
            return;
        }
        self.sync_mappings(maps);
        let live: BTreeSet<_> = maps
            .mem_slab
            .iter()
            .filter_map(|(_, mem)| mem.shared_view().map(|view| view.storage))
            .collect();
        let stale: Vec<_> = self
            .mapping_stores
            .iter()
            .filter(|(_, id)| !live.contains(id))
            .map(|(&inode, &id)| (inode, id))
            .collect();
        for (inode, id) in stale {
            self.mapping_stores.remove(&inode);
            maps.shared_stores.remove(&id);
            self.release(inode);
        }
    }
}
