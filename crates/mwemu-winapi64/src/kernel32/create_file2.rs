use crate::emu;
use crate::winapi::helper;

pub fn CreateFile2(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateFile2");
    emu.regs_mut().rax = helper::handler_create("CreateFile2");
}
