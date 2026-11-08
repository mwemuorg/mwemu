use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreatePseudoConsole(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreatePseudoConsole");
    emu.regs_mut().rax = helper::handler_create("CreatePseudoConsole");
    stdcall_ret(emu, 1);
}
