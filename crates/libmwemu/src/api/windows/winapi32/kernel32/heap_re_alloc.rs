use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::windows::constants;

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

    if old_mem == 0 || new_size_raw == 0 {
        heap_engine::fail_allocation(emu, flags);
        emu.regs_mut().rax = 0;
        return;
    }

    let mut effective_size = new_size_raw;
    if effective_size < emu.cfg.heap_alloc_min_size {
        effective_size = emu.cfg.heap_alloc_min_size;
    }

    let old_size = match heap_engine::heap_allocation_size(emu, old_mem) {
        Some(s) => s,
        None => {
            emu.regs_mut().rax = 0;
            return;
        }
    };

    if (flags & constants::HEAP_REALLOC_IN_PLACE_ONLY) != 0 {
        if effective_size <= old_size as u64 {
            emu.regs_mut().rax = old_mem;
            return;
        }
        heap_engine::fail_allocation(emu, flags);
        emu.regs_mut().rax = 0;
        return;
    }

    let new_addr = match heap_engine::heap_allocate(emu, heap_handle, effective_size) {
        Some(a) => a,
        None => {
            heap_engine::fail_allocation(emu, flags);
            emu.regs_mut().rax = 0;
            return;
        }
    };

    let copy_size = std::cmp::min(old_size, effective_size as usize);
    if !emu.maps.memcpy(new_addr, old_mem, copy_size) {
        heap_engine::heap_free(emu, heap_handle, new_addr);
        heap_engine::fail_allocation(emu, flags);
        emu.regs_mut().rax = 0;
        return;
    }

    if (flags & constants::HEAP_ZERO_MEMORY) != 0 && (effective_size as usize) > old_size {
        let zero_start = new_addr + old_size as u64;
        let zero_size = (effective_size as usize) - old_size;
        emu.maps.memset(zero_start, 0, zero_size);
    }

    if !emu.cfg.heap_free_soft {
        heap_engine::heap_free(emu, heap_handle, old_mem);
    } else {
        emu.handle_management
            .forget_heap_allocation(heap_handle, old_mem);
    }

    log_red!(
        emu,
        "kernel32!HeapReAlloc heap: 0x{:x} flags: 0x{:x} old: 0x{:x} new: 0x{:x} sz: {}",
        heap_handle,
        flags,
        old_mem,
        new_addr,
        effective_size
    );
    emu.regs_mut().rax = new_addr;
}
