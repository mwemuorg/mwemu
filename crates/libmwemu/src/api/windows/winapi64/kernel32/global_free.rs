use crate::api::windows::common::heap as heap_engine;
use crate::emu;

pub fn GlobalFree(emu: &mut emu::Emu) {
    let mem = emu.regs().rcx;

    log_red!(emu, "kernel32!GlobalFree mem: 0x{:x}", mem);

    heap_engine::heap_free(emu, 0, mem);
    emu.regs_mut().rax = 0;
}
