use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateThreadpoolWait(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateThreadpoolWait");
    emu.regs_mut().rax = helper::handler_create("CreateThreadpoolWait");
    stdcall_ret(emu, 1);
}
