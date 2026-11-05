use crate::emu;
use crate::winapi::helper;

pub fn CreateIoCompletionPort(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateIoCompletionPort");
    emu.regs_mut().rax = helper::handler_create("CreateIoCompletionPort");
}
