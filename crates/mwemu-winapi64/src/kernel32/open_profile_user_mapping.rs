use crate::emu;
use crate::winapi::helper;

pub fn OpenProfileUserMapping(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenProfileUserMapping");
    emu.regs_mut().rax = helper::handler_create("OpenProfileUserMapping");
}
