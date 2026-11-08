use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateProcessInternalW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateProcessInternalW");
    emu.regs_mut().rax = helper::handler_create("CreateProcessInternalW");
    stdcall_ret(emu, 1);
}
