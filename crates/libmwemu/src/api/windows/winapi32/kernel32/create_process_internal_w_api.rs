use crate::emu;
use crate::winapi::helper;

pub fn CreateProcessInternalW_api(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateProcessInternalW");
    emu.regs_mut().rax = helper::handler_create("CreateProcessInternalW");
}
