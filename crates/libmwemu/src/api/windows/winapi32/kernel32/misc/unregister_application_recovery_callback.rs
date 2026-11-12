use crate::emu;
use crate::windows::constants::errors::*;

pub fn UnregisterApplicationRecoveryCallback(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!UnregisterApplicationRecoveryCallback");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
