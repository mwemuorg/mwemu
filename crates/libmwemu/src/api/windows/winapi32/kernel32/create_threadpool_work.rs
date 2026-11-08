use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateThreadpoolWork(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateThreadpoolWork");
    emu.regs_mut().rax = helper::handler_create("CreateThreadpoolWork");
    stdcall_ret(emu, 1);
}
