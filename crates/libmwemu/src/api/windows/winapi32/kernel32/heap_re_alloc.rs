use crate::maps::mem64::Permission;
use crate::{emu, windows::constants};

const LARGE_ALLOC_THRESHOLD: u64 = 0x8000;
const ALLOC_MAP_PREFIX: &str = "alloc_";

enum OldKind {
    Arena { addr: u64, size: usize },
    Map { base: u64, size: usize },
    Invalid,
}

pub fn HeapReAlloc(emu: &mut emu::Emu) {
    let heap_handle = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!HeapReAlloc cannot read heap handle") as u64;
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!HeapReAlloc cannot read flags") as u64;
    let old_mem = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!HeapReAlloc cannot read lpMem") as u64;
    let new_size_raw = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("kernel32!HeapReAlloc cannot read dwBytes") as u64;

    // stdcall: caller expects us to pop all 4 args regardless of outcome.
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    log_red!(
        emu,
        "kernel32!HeapReAlloc heap: 0x{:x} flags: 0x{:x} old_mem: 0x{:x} new_size: {}",
        heap_handle,
        flags,
        old_mem,
        new_size_raw
    );

    match realloc(emu, old_mem, new_size_raw, flags) {
        Some(new_addr) => {
            log_red!(
                emu,
                "kernel32!HeapReAlloc old: 0x{:x} new: 0x{:x} =0x{:x}",
                old_mem,
                new_addr,
                new_addr
            );
            emu.regs_mut().rax = new_addr;
        }
        None => {
            log_red!(
                emu,
                "kernel32!HeapReAlloc old: 0x{:x} new_size: {} =NULL",
                old_mem,
                new_size_raw
            );
            emu.regs_mut().rax = 0;
        }
    }
}

fn realloc(emu: &mut emu::Emu, old_mem: u64, new_size_raw: u64, flags: u64) -> Option<u64> {
    if old_mem == 0 || new_size_raw == 0 {
        return None;
    }

    let mut effective_size = new_size_raw;
    if effective_size < emu.cfg.heap_alloc_min_size {
        effective_size = emu.cfg.heap_alloc_min_size;
    }

    let kind = classify_old(emu, old_mem);
    let old_size = match &kind {
        OldKind::Arena { size, .. } => *size,
        OldKind::Map { size, .. } => *size,
        OldKind::Invalid => return None,
    };

    if (flags & constants::HEAP_REALLOC_IN_PLACE_ONLY) != 0 {
        if effective_size <= old_size as u64 {
            return Some(old_mem);
        }
        return None;
    }

    let new_addr = allocate_destination(emu, effective_size)?;

    let copy_size = std::cmp::min(old_size, effective_size as usize);
    if !emu.maps.memcpy(new_addr, old_mem, copy_size) {
        free_destination(emu, new_addr, effective_size);
        return None;
    }

    if (flags & constants::HEAP_ZERO_MEMORY) != 0 && (effective_size as usize) > old_size {
        let zero_start = new_addr + old_size as u64;
        let zero_size = (effective_size as usize) - old_size;
        emu.maps.memset(zero_start, 0, zero_size);
    }

    if !emu.cfg.heap_free_soft {
        release_old(emu, &kind);
    }

    Some(new_addr)
}

fn classify_old(emu: &emu::Emu, old_mem: u64) -> OldKind {
    if let Some(heap) = emu.heap_management.as_ref() {
        if let Some(sz) = heap.allocation_size(old_mem) {
            return OldKind::Arena { addr: old_mem, size: sz };
        }
    }

    match emu.maps.get_mem_by_addr(old_mem) {
        Some(mem) => {
            let base = mem.get_base();
            if base != old_mem {
                return OldKind::Invalid;
            }
            let name = mem.get_name();
            if !name.starts_with(ALLOC_MAP_PREFIX) {
                return OldKind::Invalid;
            }
            OldKind::Map {
                base,
                size: mem.size(),
            }
        }
        None => OldKind::Invalid,
    }
}

fn allocate_destination(emu: &mut emu::Emu, size: u64) -> Option<u64> {
    if size < LARGE_ALLOC_THRESHOLD {
        let heap = emu.heap_mut();
        heap.allocate(size as usize)
    } else {
        let addr = emu.maps.alloc(size)?;
        let name = format!("{}{:x}", ALLOC_MAP_PREFIX, addr);
        if emu
            .maps
            .create_map(&name, addr, size, Permission::READ_WRITE)
            .is_err()
        {
            return None;
        }
        Some(addr)
    }
}

fn free_destination(emu: &mut emu::Emu, addr: u64, size: u64) {
    if size >= LARGE_ALLOC_THRESHOLD {
        emu.maps.dealloc(addr);
    }
}

fn release_old(emu: &mut emu::Emu, kind: &OldKind) {
    match kind {
        OldKind::Arena { addr, .. } => {
            if let Some(heap) = emu.heap_management.as_mut() {
                heap.free(*addr);
            }
        }
        OldKind::Map { base, size } => {
            if (*size as u64) >= LARGE_ALLOC_THRESHOLD {
                emu.maps.dealloc(*base);
            }
        }
        OldKind::Invalid => {}
    }
}