use crate::emu;
use crate::windows::constants::errors::*;

pub fn GetNumberOfConsoleFonts(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetNumberOfConsoleFonts");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
