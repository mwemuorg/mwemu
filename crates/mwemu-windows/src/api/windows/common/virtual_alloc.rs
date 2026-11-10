//! Helpers shared by the kernel32 `Virtual*` allocators on both architectures
//! (winapi32 and winapi64): `VirtualAlloc`, `VirtualAllocEx` and
//! `VirtualAllocExNuma`. These were previously copy-pasted into each of the six
//! allocator files; keep the single source of truth here.

use crate::windows::constants::memory::*;
/// Decode a page-protection mask into `(can_read, can_write, can_execute)`.
pub fn permissions(prot: u32) -> (bool, bool, bool) {
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
pub fn round_up(size: u64) -> u64 {
    const PAGE_SIZE: u64 = 0x1000;
    (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
}
