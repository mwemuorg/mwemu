use crate::emu;
use crate::winapi::helper;

pub fn FindFirstFileTransactedA(emu: &mut emu::Emu) {
    for _ in 0..7u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!FindFirstFileTransactedA");
    emu.regs_mut().rax = helper::handler_create("FindFirstFileTransactedA");
}

pub fn FindFirstFileTransactedW(emu: &mut emu::Emu) {
    for _ in 0..7u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!FindFirstFileTransactedW");
    emu.regs_mut().rax = helper::handler_create("FindFirstFileTransactedW");
}
