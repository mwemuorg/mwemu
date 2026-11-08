use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn OpenSemaphoreA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenSemaphoreA");
    emu.regs_mut().rax = helper::handler_create("OpenSemaphoreA");
    stdcall_ret(emu, 1);
}
