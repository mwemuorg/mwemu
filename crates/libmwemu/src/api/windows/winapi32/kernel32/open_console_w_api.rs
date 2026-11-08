use crate::emu;
use crate::winapi::helper;

pub fn OpenConsoleW_api(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenConsoleW");
    emu.regs_mut().rax = helper::handler_create("OpenConsoleW");
}
