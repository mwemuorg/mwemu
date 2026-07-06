//! Helpers shared by the kernel32 `Virtual*` allocators on both architectures
//! (winapi32 and winapi64): `VirtualAlloc`, `VirtualAllocEx` and
//! `VirtualAllocExNuma`. These were previously copy-pasted into each of the six
//! allocator files; keep the single source of truth here.

/// Page-protection bits used to derive memory permissions. Only the values that
/// feed [`permissions`] are listed (NOACCESS/GUARD/NOCACHE carry no R/W/X).
const PAGE_READONLY: u32 = 0x02;
const PAGE_READWRITE: u32 = 0x04;
const PAGE_WRITECOPY: u32 = 0x08;
const PAGE_EXECUTE: u32 = 0x10;
const PAGE_EXECUTE_READ: u32 = 0x20;
const PAGE_EXECUTE_READWRITE: u32 = 0x40;
const PAGE_EXECUTE_WRITECOPY: u32 = 0x80;

/// Decode a page-protection mask into `(can_read, can_write, can_execute)`.
pub(crate) fn permissions(prot: u32) -> (bool, bool, bool) {
    let can_read = (prot
        & (PAGE_READONLY
            | PAGE_READWRITE
            | PAGE_WRITECOPY
            | PAGE_EXECUTE_READ
            | PAGE_EXECUTE_READWRITE
            | PAGE_EXECUTE_WRITECOPY))
        != 0;
    let can_write = (prot
        & (PAGE_READWRITE | PAGE_WRITECOPY | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY))
        != 0;
    let can_execute = (prot
        & (PAGE_EXECUTE | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY))
        != 0;
    (can_read, can_write, can_execute)
}

/// Round a byte size up to the next 0x1000 page boundary.
pub(crate) fn round_up(size: u64) -> u64 {
    const PAGE_SIZE: u64 = 0x1000;
    (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
}
