use crate::emu;
use crate::winapi::helper;

pub fn CreateFileTransactedA(emu: &mut emu::Emu) {
    for _ in 0..10u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateFileTransactedA");
    emu.regs_mut().rax = helper::handler_create("CreateFileTransactedA");
}

pub fn CreateFileTransactedW(emu: &mut emu::Emu) {
    for _ in 0..10u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateFileTransactedW");
    emu.regs_mut().rax = helper::handler_create("CreateFileTransactedW");
}
