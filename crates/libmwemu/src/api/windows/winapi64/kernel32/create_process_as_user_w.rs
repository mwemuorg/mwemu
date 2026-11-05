use crate::emu;
use crate::winapi::helper;

pub fn CreateProcessAsUserW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateProcessAsUserW");
    emu.regs_mut().rax = helper::handler_create("CreateProcessAsUserW");
}
