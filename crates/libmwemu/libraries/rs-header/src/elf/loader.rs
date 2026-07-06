//! The single coupling point between `rs-header`'s ELF loader and its consumer.
//!
//! `rs-header` knows *what* to map and *which* relocations to apply; the
//! consumer provides *how* to touch its own guest memory. This mirrors
//! [`crate::pe::PeLoader`] so PE and ELF share one philosophy.

/// Runtime memory permissions for a mapped region. ELF program-header /
/// section flags are translated into this before reaching the loader, so the
/// consumer never has to know about ELF's `PF_*` / `SHF_*` bit layouts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Perm {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl Perm {
    pub const fn from_flags(read: bool, write: bool, execute: bool) -> Perm {
        Perm { read, write, execute }
    }

    pub const READ_WRITE: Perm = Perm { read: true, write: true, execute: false };
}

/// Backend the ELF loader writes through. The consumer maps regions into its
/// own address space and patches relocation slots; `rs-header` owns the parsing
/// and the decision of *what* to write.
pub trait ElfLoader {
    /// Map `size` bytes named `name` at `addr` with `perm`. Returns the base the
    /// region was actually mapped at — normally `addr`, but the implementor may
    /// relocate it elsewhere to dodge an overlap (in which case the loader keeps
    /// the returned address). Returns `None` if the region could not be mapped.
    fn map(&mut self, name: &str, addr: u64, size: u64, perm: Perm) -> Option<u64>;

    /// Reserve shared-library address space of `size`; returns its base.
    fn lib_alloc(&mut self, size: u64) -> Option<u64>;

    /// Write `data` into already-mapped memory at `addr`, bypassing the region's
    /// permissions (the loader populates read-only segments after mapping them).
    fn write_bytes(&mut self, addr: u64, data: &[u8]) -> bool;

    /// Read a little-endian qword from mapped memory, or `None` if unmapped.
    fn read_qword(&self, addr: u64) -> Option<u64>;

    /// Patch a little-endian qword in mapped memory, bypassing permissions.
    /// Returns `false` if `addr` is not mapped.
    fn write_qword(&mut self, addr: u64, val: u64) -> bool;
}
