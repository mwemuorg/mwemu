use crate::emu;
use crate::winapi::helper;

pub fn OpenFileById(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenFileById");
    emu.regs_mut().rax = helper::handler_create("OpenFileById");
}
