use crate::emu;
use crate::winapi::helper;

pub fn CreateProcessInternalA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateProcessInternalA");
    emu.regs_mut().rax = helper::handler_create("CreateProcessInternalA");
}
