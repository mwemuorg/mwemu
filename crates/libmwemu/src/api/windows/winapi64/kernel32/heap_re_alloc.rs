use crate::api::windows::common::heap as heap_engine;
use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;
pub fn HeapReAlloc(emu: &mut emu::Emu) {
    let heap_handle = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let old_mem = emu.regs().r8;
    let new_size_raw = emu.regs().r9;
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
    let new_addr = match heap_engine::heap_reallocate(emu, heap_handle, old_mem, effective_size) {
        Some(a) => a,
        None => {
            heap_engine::fail_allocation(emu, flags);
            emu.regs_mut().rax = 0;
            return;
        }
    };
    if (flags & constants::HEAP_ZERO_MEMORY) != 0 && (effective_size as usize) > old_size {
        emu.maps.memset(
            new_addr + old_size as u64,
            0,
            (effective_size as usize) - old_size,
        );
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
    let handle = helper::handler_create("HeapReAlloc");
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.regs_mut().rax = new_addr;
}
