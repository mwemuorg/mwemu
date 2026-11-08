use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateFileTransactedW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateFileTransactedW");
    emu.regs_mut().rax = helper::handler_create("CreateFileTransactedW");
    stdcall_ret(emu, 1);
}
