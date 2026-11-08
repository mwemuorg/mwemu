use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateConsoleScreenBuffer(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateConsoleScreenBuffer");
    emu.regs_mut().rax = helper::handler_create("CreateConsoleScreenBuffer");
    stdcall_ret(emu, 1);
}
