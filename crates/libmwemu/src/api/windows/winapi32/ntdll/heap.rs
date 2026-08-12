use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::helper;
use crate::windows::constants;

const ALLOC_PREFIX: &str = "alloc_";
const LARGE_ALLOC_THRESHOLD: u64 = 0x8000;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "RtlGetProcessHeaps" => RtlGetProcessHeaps(emu),
        "RtlFreeHeap" => RtlFreeHeap(emu),
        "RtlAllocateHeap" => RtlAllocateHeap(emu),
        "RtlReAllocateHeap" => RtlReAllocateHeap(emu),
        _ => return false,
    }
    true
}

fn RtlGetProcessHeaps(emu: &mut emu::Emu) {
    log_red!(emu, "ntdll!RtlGetProcessHeaps");

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}

/// Allocates from the O1Heap arena for small sizes, maps a dedicated
/// region otherwise (same threshold as kernel32!HeapAlloc).
fn allocate_memory(emu: &mut emu::Emu, size: u64) -> Option<u64> {
    if size < LARGE_ALLOC_THRESHOLD {
        let heap_manage = emu.heap_mut();
        return heap_manage.allocate(size as usize);
    }

    let allocation = emu.maps.alloc(size)?;
    emu.maps
        .create_map(
            format!("{}{:x}", ALLOC_PREFIX, allocation).as_str(),
            allocation,
            size,
            Permission::READ_WRITE,
        )
        .ok()?;
    Some(allocation)
}

/// Classification of a pointer allocated by `allocate_memory`.
enum AllocKind {
    Arena { size: usize },
    Map { base: u64, size: usize },
    Invalid,
}

fn classify(emu: &emu::Emu, addr: u64) -> AllocKind {
    if let Some(heap) = emu.heap_management.as_ref() {
        if let Some(size) = heap.allocation_size(addr) {
            return AllocKind::Arena { size };
        }
    }

    match emu.maps.get_mem_by_addr(addr) {
        Some(mem) if mem.get_base() == addr && mem.get_name().starts_with(ALLOC_PREFIX) => {
            AllocKind::Map {
                base: addr,
                size: mem.size(),
            }
        }
        _ => AllocKind::Invalid,
    }
}

/// Releases a pointer allocated by `allocate_memory` (or any alloc_ map).
/// Honors cfg.heap_free_soft.
fn release(emu: &mut emu::Emu, addr: u64) {
    if emu.cfg.heap_free_soft {
        return;
    }
    match classify(emu, addr) {
        AllocKind::Arena { .. } => {
            if let Some(heap) = emu.heap_management.as_mut() {
                heap.free(addr);
            }
        }
        AllocKind::Map { base, .. } => emu.maps.dealloc(base),
        AllocKind::Invalid => {}
    }
}

fn RtlFreeHeap(emu: &mut emu::Emu) {
    let handle = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!RtlFreeHeap error reading handle param") as u64;
    let base_addr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ntdll!RtlFreeHeap error reading base_addr param") as u64;

    log_red!(emu, "ntdll!RtlFreeHeap 0x{}", base_addr);

    helper::handler_close(handle);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    match classify(emu, base_addr) {
        AllocKind::Arena { .. } | AllocKind::Map { .. } => {
            release(emu, base_addr);
            emu.regs_mut().rax = 1;
        }
        AllocKind::Invalid => {
            emu.regs_mut().rax = 0;
            if emu.cfg.verbose >= 1 {
                log::trace!("trying to free a systems map {}", base_addr);
            }
        }
    }
}

pub fn RtlAllocateHeap(emu: &mut emu::Emu) {
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ntdll!RtlAllocateHeap error reading size param") as u64;

    let base = allocate_memory(emu, size).expect("ntdll!RtlAllocateHeap out of memory");

    log_red!(emu, "ntdll!RtlAllocateHeap sz: {} addr: 0x{:x}", size, base);

    emu.regs_mut().rax = base;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn RtlReAllocateHeap(emu: &mut emu::Emu) {
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!RtlReAllocateHeap error reading flags") as u64;
    let old_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ntdll!RtlReAllocateHeap error reading ptr") as u64;
    let new_size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("ntdll!RtlReAllocateHeap error reading size") as u64;

    // stdcall: caller expects us to pop all 4 args regardless of outcome.
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    log_red!(
        emu,
        "ntdll!RtlReAllocateHeap flags: {:x} old: 0x{:x} sz: {}",
        flags,
        old_ptr,
        new_size
    );

    if old_ptr == 0 || new_size == 0 {
        emu.regs_mut().rax = 0;
        return;
    }

    let old_size = match classify(emu, old_ptr) {
        AllocKind::Arena { size } | AllocKind::Map { size, .. } => size,
        AllocKind::Invalid => {
            emu.regs_mut().rax = 0;
            return;
        }
    };

    if (flags & constants::HEAP_REALLOC_IN_PLACE_ONLY) != 0 {
        if new_size <= old_size as u64 {
            emu.regs_mut().rax = old_ptr;
            return;
        }
        emu.regs_mut().rax = 0;
        return;
    }

    let new_addr = match allocate_memory(emu, new_size) {
        Some(a) => a,
        None => {
            emu.regs_mut().rax = 0;
            return;
        }
    };

    let copy_size = std::cmp::min(old_size, new_size as usize);
    if !emu.maps.memcpy(new_addr, old_ptr, copy_size) {
        release(emu, new_addr);
        emu.regs_mut().rax = 0;
        return;
    }

    if (flags & constants::HEAP_ZERO_MEMORY) != 0 && (new_size as usize) > old_size {
        emu.maps.memset(
            new_addr + old_size as u64,
            0,
            (new_size as usize) - old_size,
        );
    }

    release(emu, old_ptr);
    emu.regs_mut().rax = new_addr;
}
