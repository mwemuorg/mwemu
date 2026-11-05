use crate::emu;
use crate::winapi::helper;

pub fn CreateFiber(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateFiber");
    emu.regs_mut().rax = helper::handler_create("CreateFiber");
}
