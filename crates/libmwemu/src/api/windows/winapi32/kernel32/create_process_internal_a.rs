use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateProcessInternalA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateProcessInternalA");
    emu.regs_mut().rax = helper::handler_create("CreateProcessInternalA");
    stdcall_ret(emu, 1);
}
