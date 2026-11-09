use crate::emu;
use crate::winapi::helper;

pub fn CreateThreadpoolWait(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateThreadpoolWait");
    emu.regs_mut().rax = helper::handler_create("CreateThreadpoolWait");
}
