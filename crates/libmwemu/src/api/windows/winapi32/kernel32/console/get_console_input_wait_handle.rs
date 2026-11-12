use crate::emu;
use crate::windows::constants::errors::*;

pub fn GetConsoleInputWaitHandle(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetConsoleInputWaitHandle");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
