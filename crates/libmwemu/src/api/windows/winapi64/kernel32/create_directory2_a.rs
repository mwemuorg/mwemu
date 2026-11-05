use crate::emu;
use crate::winapi::helper;

pub fn CreateDirectory2A(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateDirectory2A");
    emu.regs_mut().rax = helper::handler_create("CreateDirectory2A");
}
