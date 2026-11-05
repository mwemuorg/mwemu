use crate::emu;
use crate::winapi::helper;

pub fn FindFirstFileNameW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!FindFirstFileNameW");
    emu.regs_mut().rax = helper::handler_create("FindFirstFileNameW");
}
