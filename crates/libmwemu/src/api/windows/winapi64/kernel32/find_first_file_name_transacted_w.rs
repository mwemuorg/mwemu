use crate::emu;
use crate::winapi::helper;

pub fn FindFirstFileNameTransactedW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!FindFirstFileNameTransactedW");
    emu.regs_mut().rax = helper::handler_create("FindFirstFileNameTransactedW");
}
