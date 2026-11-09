use crate::emu;
use crate::winapi::helper;

pub fn CreateThreadpoolWork(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateThreadpoolWork");
    emu.regs_mut().rax = helper::handler_create("CreateThreadpoolWork");
}
