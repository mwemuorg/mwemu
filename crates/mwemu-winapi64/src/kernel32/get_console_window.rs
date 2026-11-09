use crate::emu;
use crate::winapi::helper;

pub fn handler(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetConsoleWindow");
    let handle = helper::handler_create("console_window");
    emu.regs_mut().rax = handle;
}
