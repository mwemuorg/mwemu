use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::windows::constants;

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

    // ntdll!RtlAllocateHeap bumps tiny requests up to 1024 bytes (kernel32!HeapAlloc
    // pads to cfg.heap_alloc_min_size instead).
    if size < 1024 {
        size = 1024;
    }

    match heap_engine::heap_allocate(emu, handle, size) {
        Some(addr) => {
            log_red!(
                emu,
                "ntdll!RtlAllocateHeap hndl: 0x{:x} sz: {} addr: 0x{:x}",
                handle,
                size,
                addr
            );
            emu.regs_mut().rax = addr;
        }
        None => {
            log_red!(
                emu,
                "ntdll!RtlAllocateHeap FAILED hndl: 0x{:x} sz: {}",
                handle,
                size
            );
            heap_engine::fail_allocation(emu, flags);
            emu.regs_mut().rax = 0;
        }
    }
}

fn RtlFreeHeap(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let base_addr = emu.regs().r8;

    log_red!(
        emu,
        "ntdll!RtlFreeHeap hndl=0x{:x} base=0x{:x}",
        hndl,
        base_addr
    );

    if base_addr != 0 && heap_engine::heap_allocation_size(emu, base_addr).is_some() {
        heap_engine::heap_free(emu, hndl, base_addr);
        emu.regs_mut().rax = 1;
    } else {
        if base_addr != 0 {
            emu.handle_management
                .forget_heap_allocation(hndl, base_addr);
        }
        if emu.cfg.verbose >= 1 {
            log::trace!("trying to free a systems map {}", base_addr);
        }
        emu.regs_mut().rax = 0;
    }
}

pub fn RtlReAllocateHeap(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let old_ptr = emu.regs().r8;
    let new_size = emu.regs().r9;

    if old_ptr == 0 || new_size == 0 {
        heap_engine::fail_allocation(emu, flags);
        emu.regs_mut().rax = 0;
        return;
    }

    let old_size = match heap_engine::heap_allocation_size(emu, old_ptr) {
        Some(s) => s,
        None => {
            emu.regs_mut().rax = 0;
            return;
        }
    };

    if (flags & constants::HEAP_REALLOC_IN_PLACE_ONLY) != 0 {
        if new_size <= old_size as u64 {
            emu.regs_mut().rax = old_ptr;
            return;
        }
        heap_engine::fail_allocation(emu, flags);
        emu.regs_mut().rax = 0;
        return;
    }

    let new_addr = match heap_engine::heap_allocate(emu, handle, new_size) {
        Some(a) => a,
        None => {
            heap_engine::fail_allocation(emu, flags);
            emu.regs_mut().rax = 0;
            return;
        }
    };

    let copy_size = std::cmp::min(old_size, new_size as usize);
    if !emu.maps.memcpy(new_addr, old_ptr, copy_size) {
        heap_engine::heap_free(emu, handle, new_addr);
        heap_engine::fail_allocation(emu, flags);
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

    if !emu.cfg.heap_free_soft {
        heap_engine::heap_free(emu, handle, old_ptr);
    } else {
        emu.handle_management
            .forget_heap_allocation(handle, old_ptr);
    }

    emu.regs_mut().rax = new_addr;
}

pub fn RtlGetProcessHeaps(emu: &mut emu::Emu) {
    let count = emu.regs().rcx;
    let buffer = emu.regs().rdx;

    let keys = emu.handle_management.heap_handle_keys();
    let total = keys.len() as u64;
    log_red!(
        emu,
        "ntdll!RtlGetProcessHeaps count={} buffer=0x{:x} total={}",
        count,
        buffer,
        total
    );

    if buffer == 0 {
        emu.regs_mut().rax = total;
        return;
    }

    let to_write = std::cmp::min(count, total);
    for (i, key) in keys.iter().take(to_write as usize).enumerate() {
        let _ = emu.maps.write_qword(buffer + (i as u64) * 8, *key as u64);
    }

    if to_write < count {
        emu.regs_mut().rax = 0;
    } else {
        emu.regs_mut().rax = to_write;
    }
}

fn RtlFreeAnsiString(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;

    log_red!(emu, "ntdll!RtlFreeAnsiString 0x{}", ptr);
}
