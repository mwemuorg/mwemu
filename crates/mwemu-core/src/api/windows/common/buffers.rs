//! Tracks the returned pointer separately from its backing allocation.
use crate::emu::Emu;
use crate::maps::buffers::{BufferAllocation, BufferKind};
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::align_up;
use crate::winapi::common::heap;

const LARGE_ALLOCATION: u64 = 0x8000;

pub fn allocate(
    emu: &mut Emu,
    size: u64,
    alignment: u64,
    prefix: u64,
    kind: BufferKind,
) -> Option<u64> {
    if !alignment.is_power_of_two() {
        return None;
    }
    let total = size
        .max(1)
        .checked_add(prefix)?
        .checked_add(alignment - 1)?;
    if total > emu.maps.max_alloc_size || (!emu.cfg.arch.is_64bits() && total > u32::MAX as u64) {
        return None;
    }
    let arena = total < LARGE_ALLOCATION || emu.cfg.memory_guard;
    let base = if arena {
        heap::heap_allocate(emu, 0, total)?
    } else {
        let base = emu.maps.alloc(total)?;
        let name = emu.maps.unique_map_name(&format!("buffer_{base:x}"));
        emu.maps
            .create_map(&name, base, total, Permission::READ_WRITE)
            .ok()?;
        base
    };
    let address = align_up(base + prefix, alignment)?;
    emu.maps.buffers.insert(
        address,
        BufferAllocation {
            base,
            size,
            alignment,
            kind,
            arena,
        },
    );
    Some(address)
}

pub fn size(emu: &Emu, address: u64, kind: BufferKind) -> Option<u64> {
    emu.maps
        .buffers
        .get(&address)
        .filter(|a| a.kind == kind && emu.maps.is_mapped(a.base))
        .map(|a| a.size)
}

pub fn free(emu: &mut Emu, address: u64, kind: BufferKind) -> bool {
    if address == 0 {
        return true;
    }
    let Some(allocation) = emu
        .maps
        .buffers
        .get(&address)
        .copied()
        .filter(|a| a.kind == kind)
    else {
        return false;
    };
    if allocation.arena {
        if !heap::heap_free(emu, 0, allocation.base) {
            return false;
        }
    } else {
        if emu
            .maps
            .get_mem_by_addr(allocation.base)
            .is_none_or(|m| m.get_base() != allocation.base)
        {
            return false;
        }
        emu.maps.dealloc(allocation.base);
    }
    emu.maps.buffers.remove(&address);
    true
}

pub fn reallocate(emu: &mut Emu, address: u64, size: u64, kind: BufferKind) -> Option<u64> {
    if address == 0 {
        return allocate(emu, size, 16, 0, kind);
    }
    let previous = *emu.maps.buffers.get(&address).filter(|a| a.kind == kind)?;
    if size == 0 {
        return free(emu, address, kind).then_some(0);
    }
    let new_address = allocate(emu, size, previous.alignment, 0, kind)?;
    if !emu
        .maps
        .copy_range(new_address, address, size.min(previous.size) as usize)
    {
        free(emu, new_address, kind);
        return None;
    }
    if !free(emu, address, kind) {
        free(emu, new_address, kind);
        return None;
    }
    Some(new_address)
}
