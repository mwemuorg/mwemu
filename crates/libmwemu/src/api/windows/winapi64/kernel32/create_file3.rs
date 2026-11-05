use crate::emu;
use crate::winapi::helper;

pub fn CreateFile3(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateFile3");
    emu.regs_mut().rax = helper::handler_create("CreateFile3");
}
