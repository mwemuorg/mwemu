use crate::emu;
use crate::winapi::helper;

pub fn CreateMutexExA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateMutexExA");
    emu.regs_mut().rax = helper::handler_create("CreateMutexExA");
}
