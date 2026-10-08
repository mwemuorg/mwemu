use crate::api::windows::common::heap as heap_engine;
use crate::emu;

pub fn GlobalAlloc(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx as u32;
    let size = emu.regs().rdx;

    let zeroinit = flags & 0x0040 != 0; // GMEM_ZEROINIT

    match heap_engine::heap_allocate(emu, 0, size) {
        Some(addr) => {
            if zeroinit {
                emu.maps.memset(addr, 0, size as usize);
            }
            log_red!(
                emu,
                "kernel32!GlobalAlloc flags: 0x{:x} size: {} =0x{:x}",
                flags,
                size,
                addr
            );
            emu.regs_mut().rax = addr;
        }
        None => {
            log_red!(
                emu,
                "kernel32!GlobalAlloc failed flags: 0x{:x} size: {}",
                flags,
                size
            );
            emu.regs_mut().rax = 0;
        }
    }
}
