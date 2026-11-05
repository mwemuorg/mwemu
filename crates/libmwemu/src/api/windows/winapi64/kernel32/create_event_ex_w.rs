use crate::emu;
use crate::winapi::helper;

pub fn CreateEventExW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateEventExW");
    emu.regs_mut().rax = helper::handler_create("CreateEventExW");
}
