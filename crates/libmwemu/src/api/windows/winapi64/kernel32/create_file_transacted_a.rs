use crate::emu;
use crate::winapi::helper;

pub fn CreateFileTransactedA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateFileTransactedA");
    emu.regs_mut().rax = helper::handler_create("CreateFileTransactedA");
}
