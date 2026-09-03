//! Shared heap routing engine used by kernel32 + ntdll heap APIs.

use crate::emu;
use crate::exception::types::ExceptionType;
use crate::maps::mem64::Permission;
use crate::windows::constants;

pub(crate) const LARGE_ALLOC_THRESHOLD: u64 = 0x8000;
const ALLOC_MAP_PREFIX: &str = "alloc_";

/// Lenient allocation context for a guest heap handle. Unknown/zero handles
/// (e.g. `0x1234` from existing tests) fall back to the process arena (index 0)
/// with `maximum_size == 0` (growable).
pub(crate) fn alloc_context(emu: &emu::Emu, handle: u64) -> (usize, u64) {
    if let Some((arena, max)) = emu.handle_management.heap_alloc_context(handle) {
        (arena, max)
    } else {
        (0, 0)
    }
}

/// Classify a pointer that may have come from any arena or a dedicated `alloc_`
/// map. Replaces the per-file `classify()` helpers.
pub(crate) enum HeapPtrKind {
    Arena,
    Map,
    Invalid,
}

pub(crate) fn classify(emu: &emu::Emu, addr: u64) -> HeapPtrKind {
    for arena in emu.heap_arenas.iter() {
        if arena.check_fragment_exists(addr) {
            return HeapPtrKind::Arena;
        }
    }
    if let Some(mem) = emu.maps.get_mem_by_addr(addr) {
        if mem.get_base() == addr
            && (mem.get_name().starts_with(ALLOC_MAP_PREFIX)
                || mem.get_name().starts_with("valloc_"))
        {
            return HeapPtrKind::Map;
        }
    }
    HeapPtrKind::Invalid
}

/// Size of an allocation at `addr` (owning arena or dedicated map).
pub(crate) fn heap_allocation_size(emu: &emu::Emu, addr: u64) -> Option<usize> {
    for arena in emu.heap_arenas.iter() {
        if let Some(size) = arena.allocation_size(addr) {
            return Some(size);
        }
    }
    if let Some(mem) = emu.maps.get_mem_by_addr(addr) {
        if mem.get_base() == addr
            && (mem.get_name().starts_with(ALLOC_MAP_PREFIX)
                || mem.get_name().starts_with("valloc_"))
        {
            return Some(mem.size());
        }
    }
    None
}

/// Allocate `size` bytes for the heap identified by `handle`.
///
/// * `size >= LARGE_ALLOC_THRESHOLD` -> dedicated `alloc_` map.
/// * otherwise -> owning arena (or process arena for unknown handles).
///
/// Returns `None` on:
/// * fixed-size heap (`maximum_size != 0`) with `size > maximum_size`,
/// * arena exhaustion,
/// * large-alloc map creation failure.
pub(crate) fn heap_allocate(emu: &mut emu::Emu, handle: u64, size: u64) -> Option<u64> {
    let (arena_idx, maximum_size) = alloc_context(emu, handle);
    if maximum_size != 0 && size > maximum_size {
        return None;
    }
    if size >= LARGE_ALLOC_THRESHOLD {
        let allocation = emu.maps.alloc(size)?;
        emu.maps
            .create_map(
                format!("{}{:x}", ALLOC_MAP_PREFIX, allocation).as_str(),
                allocation,
                size,
                Permission::READ_WRITE,
            )
            .ok()?;
        emu.handle_management
            .record_heap_allocation(handle, allocation, size);
        return Some(allocation);
    }
    let addr = emu.heap_arena_mut(arena_idx)?.allocate(size as usize)?;
    emu.handle_management
        .record_heap_allocation(handle, addr, size);
    Some(addr)
}

/// Free `addr` from the owning arena or the dedicated map. Honors
/// `cfg.heap_free_soft`. Also forgets the matching allocation record.
/// Returns true if memory was released or soft-free was applied.
pub(crate) fn heap_free(emu: &mut emu::Emu, handle: u64, addr: u64) -> bool {
    emu.handle_management.forget_heap_allocation(handle, addr);
    if emu.cfg.heap_free_soft {
        return true;
    }
    match classify(emu, addr) {
        HeapPtrKind::Arena => {
            for arena in emu.heap_arenas.iter_mut() {
                if arena.check_fragment_exists(addr) {
                    arena.free(addr);
                    return true;
                }
            }
            false
        }
        HeapPtrKind::Map => {
            emu.maps.dealloc(addr);
            true
        }
        HeapPtrKind::Invalid => false,
    }
}

/// WinAPI failure semantics for `HeapAlloc`/`HeapReAlloc`: rax=0; if
/// `flags & HEAP_GENERATE_EXCEPTIONS`, raise `STATUS_NO_MEMORY` via the
/// exception machinery.
pub(crate) fn fail_allocation(emu: &mut emu::Emu, flags: u64) {
    if flags & constants::HEAP_GENERATE_EXCEPTIONS != 0 {
        emu.exception(ExceptionType::HeapNoMemory);
    }
}
