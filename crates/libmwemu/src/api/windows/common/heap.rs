//! Shared heap routing engine used by kernel32 + ntdll heap APIs.
//!
//! Every allocation lives inside the arena owned by its heap handle; there
//! is no dedicated-map allocation path.

use crate::emu;
use crate::exception::types::ExceptionType;
use crate::maps::mem64::Permission;
use crate::windows::constants;

/// Ensure the guest-visible heap handle for slab key `key` is a real, mapped
/// address rather than an opaque small integer, and return it.
///
/// Real Windows heap handles ARE the heap's base address. Guests that only
/// ever pass the handle back into HeapAlloc/HeapFree never notice the
/// difference, but some (packers/protectors especially) validate a handle
/// by reading bytes near it as an anti-tamper/anti-emulation check -- that
/// faults against a bare `1` or `3`. Idempotent: the map is created once per
/// key and its address cached on the `HeapHandle` itself.
pub(crate) fn heap_handle_address(emu: &mut emu::Emu, key: u32) -> u64 {
    if let Some(addr) = emu.handle_management.heap_handle_addr(key) {
        return addr;
    }
    const SLOT_SIZE: u64 = 0x1000;
    let addr = emu.maps.alloc(SLOT_SIZE).unwrap_or(0);
    if addr != 0 {
        let name = format!("heap_handle_{key:x}");
        let _ = emu
            .maps
            .create_map(&name, addr, SLOT_SIZE, Permission::READ_WRITE);
    }
    emu.handle_management.set_heap_handle_addr(key, addr);
    addr
}

/// Lenient allocation context for a guest heap handle. Unknown/zero handles
/// (e.g. `0x1234` from existing tests) fall back to the process arena (index 0)
/// with `maximum_size == 0` (growable). A `handle` that is a real address
/// handed out by `heap_handle_address` is translated back to its internal
/// slab key first.
pub(crate) fn alloc_context(emu: &emu::Emu, handle: u64) -> (usize, u64) {
    let resolved = emu
        .handle_management
        .key_for_heap_addr(handle)
        .map(|k| k as u64)
        .unwrap_or(handle);
    if let Some((arena, max)) = emu.handle_management.heap_alloc_context(resolved) {
        (arena, max)
    } else {
        (0, 0)
    }
}

/// Size of the active allocation at `addr` in whichever arena owns it.
pub(crate) fn heap_allocation_size(emu: &emu::Emu, addr: u64) -> Option<usize> {
    for arena in emu.heap_arenas.iter() {
        if let Some(size) = arena.allocation_size(addr) {
            return Some(size);
        }
    }
    None
}

/// Allocate `size` bytes inside the arena owned by `handle`.
///
/// * `size == 0` or over `usize::MAX` -> `None`,
/// * fixed-size heap (`maximum_size != 0`) with `size > maximum_size` -> `None`,
/// * arena exhaustion -> `None`.
pub(crate) fn heap_allocate(emu: &mut emu::Emu, handle: u64, size: u64) -> Option<u64> {
    if size == 0 || size > usize::MAX as u64 {
        return None;
    }
    let (arena_idx, maximum_size) = alloc_context(emu, handle);
    if maximum_size != 0 && size > maximum_size {
        return None;
    }
    emu.heap_arena_mut(arena_idx)?.allocate(size as usize)
}

/// Free `addr` only when it is an active allocation of the arena owned by
/// `handle`. Returns true when the block was released.
pub(crate) fn heap_free(emu: &mut emu::Emu, handle: u64, addr: u64) -> bool {
    let (arena_idx, _) = alloc_context(emu, handle);
    let Some(heap) = emu.heap_arena_mut(arena_idx) else {
        return false;
    };
    if !heap.check_fragment_exists(addr) {
        return false;
    }
    heap.free(addr);
    true
}

/// Reallocate `addr` to `new_size` inside the arena owned by `handle`.
/// Grows/shrinks in place when the fragment neighbors allow; otherwise the
/// allocator moves the block and the caller copies `copy_size` bytes.
pub(crate) fn heap_reallocate(
    emu: &mut emu::Emu,
    handle: u64,
    addr: u64,
    new_size: u64,
) -> Option<u64> {
    if new_size == 0 || new_size > usize::MAX as u64 {
        return None;
    }
    let (arena_idx, maximum_size) = alloc_context(emu, handle);
    if maximum_size != 0 && new_size > maximum_size {
        return None;
    }
    let heap = emu.heap_arena_mut(arena_idx)?;
    if !heap.check_fragment_exists(addr) {
        return None;
    }
    let result = heap.reallocate(addr, new_size as usize)?;
    if result.copy_size > 0 && !emu.maps.memcpy(result.new_addr, addr, result.copy_size) {
        return None;
    }
    Some(result.new_addr)
}

/// WinAPI failure semantics for `HeapAlloc`/`HeapReAlloc`: rax=0; if
/// `flags & HEAP_GENERATE_EXCEPTIONS`, raise `STATUS_NO_MEMORY` via the
/// exception machinery.
pub(crate) fn fail_allocation(emu: &mut emu::Emu, flags: u64) {
    if flags & constants::HEAP_GENERATE_EXCEPTIONS != 0 {
        emu.exception(ExceptionType::HeapNoMemory);
    }
}
