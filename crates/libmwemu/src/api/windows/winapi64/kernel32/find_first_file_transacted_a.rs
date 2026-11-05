use crate::emu;
use crate::winapi::helper;

pub fn FindFirstFileTransactedA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!FindFirstFileTransactedA");
    emu.regs_mut().rax = helper::handler_create("FindFirstFileTransactedA");
}
