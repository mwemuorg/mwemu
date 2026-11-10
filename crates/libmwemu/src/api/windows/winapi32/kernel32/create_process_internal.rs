use crate::emu;
use crate::winapi::helper;

pub fn CreateProcessInternalA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateProcessInternalA");
    emu.regs_mut().rax = helper::handler_create("CreateProcessInternalA");
}

pub fn CreateProcessInternalW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateProcessInternalW");
    emu.regs_mut().rax = helper::handler_create("CreateProcessInternalW");
}
