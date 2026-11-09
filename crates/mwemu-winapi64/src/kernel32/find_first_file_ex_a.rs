use crate::emu;
use crate::winapi::helper;

pub fn FindFirstFileExA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!FindFirstFileExA");
    emu.regs_mut().rax = helper::handler_create("FindFirstFileExA");
}
