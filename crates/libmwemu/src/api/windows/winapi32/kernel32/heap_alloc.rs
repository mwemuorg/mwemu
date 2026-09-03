use crate::api::windows::common::heap as heap_engine;
use crate::emu;

pub fn HeapAlloc(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!HeapAlloc cannot read the handle");
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!HeapAlloc cannot read the flags");
    let mut size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!HeapAlloc cannot read the size") as u64;

    // Apply minimum padding
    if size < emu.cfg.heap_alloc_min_size {
        size = emu.cfg.heap_alloc_min_size;
    }

    match heap_engine::heap_allocate(emu, hndl as u64, size) {
        Some(heap_addr) => {
            emu.regs_mut().rax = heap_addr;
            log_red!(
                emu,
                "kernel32!HeapAlloc eip: 0x{:x} flags: 0x{:x} size: {} =0x{:x}",
                emu.regs().get_eip(),
                flags,
                size,
                emu.regs().rax as u32
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
            heap_engine::fail_allocation(emu, flags as u64);
            emu.regs_mut().rax = 0;
        }
    }

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
