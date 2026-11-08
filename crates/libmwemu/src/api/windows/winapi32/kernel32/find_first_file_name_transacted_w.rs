use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn FindFirstFileNameTransactedW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!FindFirstFileNameTransactedW");
    emu.regs_mut().rax = helper::handler_create("FindFirstFileNameTransactedW");
    stdcall_ret(emu, 1);
}
