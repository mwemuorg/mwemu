use crate::emu;
use crate::winapi::helper;

pub fn OpenThreadToken(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenThreadToken");
    emu.regs_mut().rax = helper::handler_create("OpenThreadToken");
}
