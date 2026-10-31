//! Shared body of `VirtualProtect`, `VirtualProtectEx` and
//! `NtProtectVirtualMemory`: change the permissions of the mapped regions that
//! cover a range and report the previous page protection.
//!
//! Permissions live on whole mapped regions (a PE section, an allocation), so a
//! range that covers only part of a region changes the whole region.

use super::virtual_alloc::{permissions, round_up};
use crate::maps::Maps;
use crate::maps::mem64::Permission;

const PAGE_NOACCESS: u32 = 0x01;
const PAGE_READONLY: u32 = 0x02;
const PAGE_READWRITE: u32 = 0x04;
const PAGE_EXECUTE: u32 = 0x10;
const PAGE_EXECUTE_READ: u32 = 0x20;
const PAGE_EXECUTE_READWRITE: u32 = 0x40;

/// Apply `prot` to every region in `[addr, addr + size)`. Returns the page
/// protection the first page had, or None (nothing changed) when part of the
/// range is not mapped.
pub(crate) fn protect(maps: &mut Maps, addr: u64, size: u64, prot: u32) -> Option<u32> {
    let start = addr & !0xfff;
    let end = round_up(addr + size.max(1));

    let mut bases = Vec::new();
    let mut cur = start;
    while cur < end {
        let mem = maps.get_mem_by_addr(cur)?;
        if mem.get_base() < start || mem.get_bottom() > end {
            log::trace!(
                "protect 0x{:x}..0x{:x} changes the whole region {} 0x{:x}..0x{:x}",
                start,
                end,
                mem.get_name(),
                mem.get_base(),
                mem.get_bottom()
            );
        }
        bases.push(mem.get_base());
        cur = mem.get_bottom();
    }

    let old = page_protection(maps.get_mem_by_addr(start)?.permission());
    let (read, write, execute) = permissions(prot);
    let permission = Permission::from_flags(read, write, execute);
    for base in bases {
        if let Some(mem) = maps.get_mem_by_addr_mut(base) {
            mem.set_permission(permission);
        }
    }
    Some(old)
}

/// PAGE_* value describing a region permission.
fn page_protection(permission: Permission) -> u32 {
    match (
        permission.can_read(),
        permission.can_write(),
        permission.can_execute(),
    ) {
        (_, true, true) => PAGE_EXECUTE_READWRITE,
        (true, false, true) => PAGE_EXECUTE_READ,
        (false, false, true) => PAGE_EXECUTE,
        (_, true, false) => PAGE_READWRITE,
        (true, false, false) => PAGE_READONLY,
        (false, false, false) => PAGE_NOACCESS,
    }
}
