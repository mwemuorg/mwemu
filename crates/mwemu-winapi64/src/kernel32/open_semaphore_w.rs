use crate::emu;
use crate::winapi::helper;

pub fn OpenSemaphoreW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenSemaphoreW");
    emu.regs_mut().rax = helper::handler_create("OpenSemaphoreW");
}
