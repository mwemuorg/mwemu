use crate::api::windows::common::heap as heap_engine;
use crate::emu;

pub fn HeapSize(emu: &mut emu::Emu) {
    let _heap = emu.regs().rcx;
    let _flags = emu.regs().rdx;
    let mem = emu.regs().r8;

    match heap_engine::heap_allocation_size(emu, mem) {
        Some(size) => {
            log_red!(emu, "kernel32!HeapSize mem: 0x{:x} ={}", mem, size);
            emu.regs_mut().rax = size as u64;
        }
        None => {
            log_red!(emu, "kernel32!HeapSize mem: 0x{:x} not found", mem);
            emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
        }
    }
}
