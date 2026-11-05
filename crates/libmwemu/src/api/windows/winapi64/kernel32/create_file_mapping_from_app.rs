use crate::emu;
use crate::winapi::helper;

pub fn CreateFileMappingFromApp(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateFileMappingFromApp");
    emu.regs_mut().rax = helper::handler_create("CreateFileMappingFromApp");
}
