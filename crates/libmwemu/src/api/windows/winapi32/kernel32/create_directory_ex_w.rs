use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateDirectoryExW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateDirectoryExW");
    emu.regs_mut().rax = helper::handler_create("CreateDirectoryExW");
    stdcall_ret(emu, 1);
}
