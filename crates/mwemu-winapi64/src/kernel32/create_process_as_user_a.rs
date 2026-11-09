use crate::emu;
use crate::winapi::helper;

pub fn CreateProcessAsUserA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateProcessAsUserA");
    emu.regs_mut().rax = helper::handler_create("CreateProcessAsUserA");
}
