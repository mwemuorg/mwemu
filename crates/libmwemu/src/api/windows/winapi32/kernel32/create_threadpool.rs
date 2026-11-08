use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateThreadpool(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateThreadpool");
    emu.regs_mut().rax = helper::handler_create("CreateThreadpool");
    stdcall_ret(emu, 1);
}
