use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::helper;
use crate::windows::constants;

const LARGE_ALLOC_THRESHOLD: u64 = 0x8000;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        // kernel32 Heap* are forwarders to these Rtl workers (same arg layout:
        // rcx=heap, rdx=flags, r8=ptr/size), so GetProcAddress-resolved callers
        // that land on the ntdll target hit the same handlers.
        "RtlAllocateHeap" | "HeapAlloc" => RtlAllocateHeap(emu),
        "RtlFreeHeap" | "HeapFree" => RtlFreeHeap(emu),
        "RtlReAllocateHeap" | "HeapReAlloc" => RtlReAllocateHeap(emu),
        "RtlGetProcessHeaps" => RtlGetProcessHeaps(emu),
        "RtlFreeAnsiString" => RtlFreeAnsiString(emu),
        _ => return false,
    }
    true
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
            format!("alloc_{:x}", allocation).as_str(),
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
        Some(mem) if mem.get_base() == addr => {
            let name = mem.get_name();
            if name.starts_with("alloc_") || name.starts_with("valloc_") {
                return AllocKind::Map {
                    base: addr,
                    size: mem.size(),
                };
            }
            AllocKind::Invalid
        }
        _ => AllocKind::Invalid,
    }
}

/// Releases a pointer allocated by `allocate_memory` (or any alloc_/valloc_ map).
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

pub fn RtlAllocateHeap(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let mut size = emu.regs().r8;

    if size < 1024 {
        size = 1024
    }
    let alloc_addr = match allocate_memory(emu, size) {
        Some(a) => a,
        None => {
            log::warn!("/!\\ out of memory cannot allocate ntdll!RtlAllocateHeap");
            return;
        }
    };

    log_red!(
        emu,
        "ntdll!RtlAllocateHeap  hndl: {:x} sz: {}   =addr: 0x{:x}",
        handle,
        size,
        alloc_addr
    );

    emu.regs_mut().rax = alloc_addr;
}

fn RtlFreeHeap(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let base_addr = emu.regs().r8;

    log_red!(emu, "ntdll!RtlFreeHeap 0x{}", base_addr);

    helper::handler_close(hndl);

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

pub fn RtlReAllocateHeap(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let old_ptr = emu.regs().r8;
    let new_size = emu.regs().r9;

    log_red!(
        emu,
        "ntdll!RtlReAllocateHeap hndl: {:x} flags: 0x{:x} old: 0x{:x} sz: {}",
        hndl,
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

fn RtlGetProcessHeaps(emu: &mut emu::Emu) {
    let num_of_heaps = emu.regs().rcx;
    let out_process_heaps = emu.regs().rcx;

    log_red!(
        emu,
        "ntdll!RtlGetProcessHeaps num: {} out: 0x{:x}",
        num_of_heaps,
        out_process_heaps
    );

    emu.regs_mut().rax = 1;
}

fn RtlFreeAnsiString(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;

    log_red!(emu, "ntdll!RtlFreeAnsiString 0x{}", ptr);
}
