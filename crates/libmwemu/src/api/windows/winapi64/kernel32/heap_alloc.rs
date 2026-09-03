use crate::api::windows::common::heap as heap_engine;
use crate::emu;

pub fn HeapAlloc(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let mut size = emu.regs().r8;

    // Apply minimum padding
    if size < emu.cfg.heap_alloc_min_size {
        size = emu.cfg.heap_alloc_min_size;
    }

    match heap_engine::heap_allocate(emu, hndl, size) {
        Some(heap_addr) => {
            emu.regs_mut().rax = heap_addr;
            log_red!(
                emu,
                "kernel32!HeapAlloc rip: 0x{:x} flags: 0x{:x} size: {} =0x{:x}",
                emu.regs().rip,
                flags,
                size,
                emu.regs().rax
            );
        }
        None => {
            log_red!(
                emu,
                "kernel32!HeapAlloc failed (hndl=0x{:x} flags=0x{:x} size={})",
                hndl,
                flags,
                size
            );
            heap_engine::fail_allocation(emu, flags);
            emu.regs_mut().rax = 0;
        }
    }
}
