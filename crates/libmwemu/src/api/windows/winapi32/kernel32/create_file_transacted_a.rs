use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateFileTransactedA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateFileTransactedA");
    emu.regs_mut().rax = helper::handler_create("CreateFileTransactedA");
    stdcall_ret(emu, 1);
}
