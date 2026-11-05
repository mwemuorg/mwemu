use crate::emu;
use crate::winapi::helper;

pub fn CreateFileTransactedW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateFileTransactedW");
    emu.regs_mut().rax = helper::handler_create("CreateFileTransactedW");
}
