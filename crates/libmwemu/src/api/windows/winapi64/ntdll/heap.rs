use crate::emu;
use crate::maps::mem64::Permission;
use crate::winapi::helper;
use crate::windows::constants;

const VALLOC_PREFIX: &str = "valloc_";

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

pub fn RtlAllocateHeap(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let mut size = emu.regs().r8;

    if size < 1024 {
        size = 1024
    }
    let alloc_addr = match emu.maps.alloc(size) {
        Some(a) => a,
        None => {
            log::warn!("/!\\ out of memory cannot allocate ntdll!RtlAllocateHeap");
            return;
        }
    };

    let map_name = format!("{}{:x}", VALLOC_PREFIX, alloc_addr);
    emu.maps
        .create_map(&map_name, alloc_addr, size, Permission::READ_WRITE)
        .expect("ntdll!RtlAllocateHeap cannot create map");

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
    let flags = emu.regs().rdx;
    let base_addr = emu.regs().r8;

    log_red!(emu, "ntdll!RtlFreeHeap 0x{}", base_addr);

    helper::handler_close(hndl);
    let name = emu.maps.get_addr_name(base_addr).unwrap_or("").to_string();
    if name.is_empty() {
        if emu.cfg.verbose >= 1 {
            log::trace!("map not allocated, so cannot free it.");
        }
        emu.regs_mut().rax = 0;
        return;
    }

    if name.starts_with(VALLOC_PREFIX) || name.starts_with("alloc_") {
        emu.maps.dealloc(base_addr);
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
        if emu.cfg.verbose >= 1 {
            log::trace!("trying to free a systems map {}", name);
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

    // Validate the old pointer: must be the exact base of an ntdll-owned map.
    if old_ptr == 0 || new_size == 0 {
        emu.regs_mut().rax = 0;
        return;
    }

    let old_mem = match emu.maps.get_mem_by_addr(old_ptr) {
        Some(m) => m,
        None => {
            emu.regs_mut().rax = 0;
            return;
        }
    };
    let old_base = old_mem.get_base();
    let old_size = old_mem.size();
    let old_name = old_mem.get_name().to_string();
    if old_base != old_ptr
        || (!old_name.starts_with(VALLOC_PREFIX) && !old_name.starts_with("alloc_"))
    {
        emu.regs_mut().rax = 0;
        return;
    }

    if (flags & constants::HEAP_REALLOC_IN_PLACE_ONLY) != 0 {
        if new_size <= old_size as u64 {
            emu.regs_mut().rax = old_ptr;
            return;
        }
        emu.regs_mut().rax = 0;
        return;
    }

    let new_addr = match emu.maps.alloc(new_size) {
        Some(a) => a,
        None => {
            emu.regs_mut().rax = 0;
            return;
        }
    };
    let new_name = format!("{}{:x}", VALLOC_PREFIX, new_addr);
    if emu
        .maps
        .create_map(&new_name, new_addr, new_size, Permission::READ_WRITE)
        .is_err()
    {
        emu.regs_mut().rax = 0;
        return;
    }

    let copy_size = std::cmp::min(old_size, new_size as usize);
    if !emu.maps.memcpy(new_addr, old_ptr, copy_size) {
        emu.maps.dealloc(new_addr);
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

    emu.maps.dealloc(old_ptr);
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
