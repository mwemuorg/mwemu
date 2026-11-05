use crate::emu;
use crate::winapi::helper;

pub fn CreateThreadpoolTimer(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateThreadpoolTimer");
    emu.regs_mut().rax = helper::handler_create("CreateThreadpoolTimer");
}
