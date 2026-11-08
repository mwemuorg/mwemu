use crate::emu;
use crate::winapi::helper;

pub fn CreatePseudoConsole_api(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreatePseudoConsole");
    emu.regs_mut().rax = helper::handler_create("CreatePseudoConsole");
}
