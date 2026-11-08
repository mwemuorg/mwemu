use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn FindFirstFileTransactedW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!FindFirstFileTransactedW");
    emu.regs_mut().rax = helper::handler_create("FindFirstFileTransactedW");
    stdcall_ret(emu, 1);
}
