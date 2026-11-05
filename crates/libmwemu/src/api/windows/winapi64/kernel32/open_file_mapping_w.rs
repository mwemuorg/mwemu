use crate::emu;
use crate::winapi::helper;

pub fn OpenFileMappingW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenFileMappingW");
    emu.regs_mut().rax = helper::handler_create("OpenFileMappingW");
}
