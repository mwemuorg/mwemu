use crate::emu;
use crate::winapi::helper;

pub fn CreateMutexExW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateMutexExW");
    emu.regs_mut().rax = helper::handler_create("CreateMutexExW");
}
