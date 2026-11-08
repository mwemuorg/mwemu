use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateProcessAsUserA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateProcessAsUserA");
    emu.regs_mut().rax = helper::handler_create("CreateProcessAsUserA");
    stdcall_ret(emu, 1);
}
