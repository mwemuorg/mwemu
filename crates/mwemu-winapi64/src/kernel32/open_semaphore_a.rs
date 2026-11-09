use crate::emu;
use crate::winapi::helper;

pub fn OpenSemaphoreA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenSemaphoreA");
    emu.regs_mut().rax = helper::handler_create("OpenSemaphoreA");
}
