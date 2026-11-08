use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn OpenFileById(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenFileById");
    emu.regs_mut().rax = helper::handler_create("OpenFileById");
    stdcall_ret(emu, 1);
}
