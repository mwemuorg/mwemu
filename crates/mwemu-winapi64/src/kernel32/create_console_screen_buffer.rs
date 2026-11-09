use crate::emu;
use crate::winapi::helper;

pub fn CreateConsoleScreenBuffer(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateConsoleScreenBuffer");
    emu.regs_mut().rax = helper::handler_create("CreateConsoleScreenBuffer");
}
