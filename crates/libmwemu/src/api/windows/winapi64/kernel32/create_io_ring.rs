use crate::emu;
use crate::winapi::helper;

pub fn CreateIoRing(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateIoRing");
    emu.regs_mut().rax = helper::handler_create("CreateIoRing");
}
