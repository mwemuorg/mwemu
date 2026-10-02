use crate::api::windows::common::heap as heap_engine;
use crate::emu;

pub fn HeapFree(emu: &mut emu::Emu) {
    let heap = emu.regs().rcx;
    let _flags = emu.regs().rdx;
    let mem = emu.regs().r8;

    log_red!(emu, "kernel32!HeapFree mem: 0x{:x}", mem);

    if emu.cfg.memory_guard {
        emu.kernel_free(mem, "HeapFree");
    } else {
        heap_engine::heap_free(emu, heap, mem);
    }
    emu.regs_mut().rax = 1;
}
