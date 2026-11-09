use crate::emu;
use crate::windows::constants::errors::*;

pub fn UnregisterApplicationRestart(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!UnregisterApplicationRestart");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
