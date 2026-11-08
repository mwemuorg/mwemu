use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateThreadpoolTimer(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateThreadpoolTimer");
    emu.regs_mut().rax = helper::handler_create("CreateThreadpoolTimer");
    stdcall_ret(emu, 1);
}
