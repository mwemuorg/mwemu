use crate::emu;
use crate::winapi::helper;

pub fn GetConsoleInputWaitHandle(emu: &mut emu::Emu) {
    let handle = helper::handler_create("console_wait://");
    log_red!(emu, "kernel32!GetConsoleInputWaitHandle => 0x{:x}", handle);
    emu.regs_mut().rax = handle;
}
