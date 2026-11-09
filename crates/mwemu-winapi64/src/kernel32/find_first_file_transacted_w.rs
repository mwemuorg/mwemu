use crate::emu;
use crate::winapi::helper;

pub fn FindFirstFileTransactedW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!FindFirstFileTransactedW");
    emu.regs_mut().rax = helper::handler_create("FindFirstFileTransactedW");
}
