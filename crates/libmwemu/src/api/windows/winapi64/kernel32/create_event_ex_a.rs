use crate::emu;
use crate::winapi::helper;

pub fn CreateEventExA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateEventExA");
    emu.regs_mut().rax = helper::handler_create("CreateEventExA");
}
