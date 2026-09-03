use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::windows::constants;

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

pub fn RtlGetProcessHeaps(emu: &mut emu::Emu) {
    let count = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!RtlGetProcessHeaps cannot read count") as u64;
    let buffer = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!RtlGetProcessHeaps cannot read buffer") as u64;

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
        emu.stack_pop32(false);
        emu.stack_pop32(false);
        emu.regs_mut().rax = total;
        return;
    }

    let to_write = std::cmp::min(count, total);
    for (i, key) in keys.iter().take(to_write as usize).enumerate() {
        let _ = emu.maps.write_dword(buffer + (i as u64) * 4, *key);
    }

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    if to_write < count {
        emu.regs_mut().rax = 0;
    } else {
        emu.regs_mut().rax = to_write;
    }
}

pub fn RtlAllocateHeap(emu: &mut emu::Emu) {
    let handle = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!RtlAllocateHeap cannot read handle param") as u64;
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!RtlAllocateHeap cannot read flags param") as u64;
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ntdll!RtlAllocateHeap cannot read size param") as u64;

    let addr = match heap_engine::heap_allocate(emu, handle, size) {
        Some(a) => a,
        None => {
            heap_engine::fail_allocation(emu, flags);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
            return;
        }
    };

    log_red!(
        emu,
        "ntdll!RtlAllocateHeap hndl: 0x{:x} sz: {} addr: 0x{:x}",
        handle,
        size,
        addr
    );

    emu.regs_mut().rax = addr;
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn RtlFreeHeap(emu: &mut emu::Emu) {
    let handle = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!RtlFreeHeap cannot read handle param") as u64;
    let base_addr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ntdll!RtlFreeHeap cannot read base_addr param") as u64;

    log_red!(
        emu,
        "ntdll!RtlFreeHeap hndl=0x{:x} base=0x{}",
        handle,
        base_addr
    );

    heap_engine::heap_free(emu, handle, base_addr);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}

pub fn RtlReAllocateHeap(emu: &mut emu::Emu) {
    let handle = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!RtlReAllocateHeap cannot read handle param") as u64;
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!RtlReAllocateHeap cannot read flags param") as u64;
    let old_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ntdll!RtlReAllocateHeap cannot read ptr param") as u64;
    let new_size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("ntdll!RtlReAllocateHeap cannot read size param") as u64;

    // stdcall: caller expects us to pop all 4 args regardless of outcome.
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

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
