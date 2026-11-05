use crate::emu;
use crate::winapi::helper;

pub fn CreateRemoteThreadEx(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateRemoteThreadEx");
    emu.regs_mut().rax = helper::handler_create("CreateRemoteThreadEx");
}
