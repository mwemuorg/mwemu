use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn FindFirstFileTransactedA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!FindFirstFileTransactedA");
    emu.regs_mut().rax = helper::handler_create("FindFirstFileTransactedA");
    stdcall_ret(emu, 1);
}
