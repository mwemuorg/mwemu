use crate::emu;
use crate::windows::constants::errors::*;

pub fn IsTerminalServerCompatible(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!IsTerminalServerCompatible");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
