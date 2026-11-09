use crate::emu;
use crate::winapi::helper;

pub fn CreateFileMappingNumaW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateFileMappingNumaW");
    emu.regs_mut().rax = helper::handler_create("CreateFileMappingNumaW");
}
