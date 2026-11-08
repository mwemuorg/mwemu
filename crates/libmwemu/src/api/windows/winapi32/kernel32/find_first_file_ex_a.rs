use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn FindFirstFileExA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!FindFirstFileExA");
    emu.regs_mut().rax = helper::handler_create("FindFirstFileExA");
    stdcall_ret(emu, 1);
}
