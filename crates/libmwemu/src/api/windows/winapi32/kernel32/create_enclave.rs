use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateEnclave(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateEnclave");
    emu.regs_mut().rax = helper::handler_create("CreateEnclave");
    stdcall_ret(emu, 8);
}
