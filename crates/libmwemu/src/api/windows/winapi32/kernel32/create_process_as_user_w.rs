use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateProcessAsUserW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateProcessAsUserW");
    emu.regs_mut().rax = helper::handler_create("CreateProcessAsUserW");
    stdcall_ret(emu, 1);
}
