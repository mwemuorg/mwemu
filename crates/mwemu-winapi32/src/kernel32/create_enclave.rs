use crate::emu;
use crate::winapi::helper;

pub fn CreateEnclave(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateEnclave");
    emu.regs_mut().rax = helper::handler_create("CreateEnclave");
}
