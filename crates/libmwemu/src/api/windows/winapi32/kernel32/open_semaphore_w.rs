use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn OpenSemaphoreW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenSemaphoreW");
    emu.regs_mut().rax = helper::handler_create("OpenSemaphoreW");
    stdcall_ret(emu, 1);
}
