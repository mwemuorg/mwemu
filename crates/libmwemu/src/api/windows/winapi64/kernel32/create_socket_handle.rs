use crate::emu;
use crate::winapi::helper;

pub fn CreateSocketHandle(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateSocketHandle");

    emu.regs_mut().rax = helper::handler_create("CreateSocketHandle");
}
